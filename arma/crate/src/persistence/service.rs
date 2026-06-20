use super::{
    hydrate_cache,
    model::{PersistenceMetrics, PersistenceStatus, WriteOp},
    surreal::{self, SurrealRepository},
};
use crate::{RUNTIME, config::DatabaseConfig, log};
use std::{
    sync::{Mutex, atomic::Ordering},
    time::Duration,
};
use tokio::sync::mpsc;

pub struct PersistenceService {
    sender: Mutex<Option<mpsc::Sender<WriteOp>>>,
    metrics: PersistenceMetrics,
}

impl PersistenceService {
    pub fn new() -> Self {
        Self {
            sender: Mutex::new(None),
            metrics: PersistenceMetrics::default(),
        }
    }

    pub fn start(&self, config: DatabaseConfig) {
        if self.metrics.enabled.swap(true, Ordering::Relaxed) {
            return;
        }

        let capacity = config.channel_capacity.max(1);
        let (sender, receiver) = mpsc::channel(capacity);
        if let Ok(mut slot) = self.sender.lock() {
            *slot = Some(sender);
        }

        log::info(format_args!(
            "surrealdb persistence enabled: endpoint={} namespace={} database={} config={}",
            config.endpoint,
            config.namespace,
            config.database,
            crate::config::path().display()
        ));

        RUNTIME.spawn(worker(config, receiver));
    }

    pub fn enqueue(&self, op: WriteOp) {
        if !self.metrics.enabled.load(Ordering::Relaxed) {
            return;
        }

        let Ok(slot) = self.sender.lock() else {
            self.metrics.dropped.fetch_add(1, Ordering::Relaxed);
            return;
        };
        let Some(sender) = slot.as_ref() else {
            self.metrics.dropped.fetch_add(1, Ordering::Relaxed);
            return;
        };

        match sender.try_send(op) {
            Ok(()) => {
                self.metrics.queued.fetch_add(1, Ordering::Relaxed);
            }
            Err(error) => {
                self.metrics.dropped.fetch_add(1, Ordering::Relaxed);
                log::error(format_args!("surrealdb write queue dropped op: {error}"));
            }
        }
    }

    pub fn status(&self) -> PersistenceStatus {
        self.metrics.status()
    }
}

async fn worker(config: DatabaseConfig, mut receiver: mpsc::Receiver<WriteOp>) {
    let mut repository = connect_with_retry(&config).await;
    hydrate(&repository).await;

    while let Some(op) = receiver.recv().await {
        super::PERSISTENCE_SERVICE
            .metrics
            .queued
            .fetch_sub(1, Ordering::Relaxed);

        let mut delay_ms = config.reconnect_initial_ms.max(1);
        let max_delay_ms = config.reconnect_max_ms.max(delay_ms);
        loop {
            match repository.apply(&op).await {
                Ok(()) => {
                    log_applied(&op);
                    break;
                }
                Err(error) => {
                    super::PERSISTENCE_SERVICE
                        .metrics
                        .connected
                        .store(false, Ordering::Relaxed);
                    log::error(format_args!(
                        "surrealdb write failed: {error}; retrying in {delay_ms}ms"
                    ));
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                    repository = connect_with_retry(&config).await;
                    delay_ms = (delay_ms.saturating_mul(2)).min(max_delay_ms);
                }
            }
        }
    }
}

fn log_applied(op: &WriteOp) {
    match op {
        WriteOp::Upsert { table, id, .. } => {
            log::debug_in(table, format_args!("database save completed key={id}"));
        }
        WriteOp::Delete { table, id } => {
            log::debug_in(table, format_args!("database delete completed key={id}"));
        }
        WriteOp::Batch { ops } => {
            log::debug(format_args!(
                "database transaction completed operations={}",
                ops.len()
            ));
            for operation in ops {
                match operation {
                    WriteOp::Upsert { table, id, .. } => {
                        log::debug_in(table, format_args!("transaction save completed key={id}"))
                    }
                    WriteOp::Delete { table, id } => {
                        log::debug_in(table, format_args!("transaction delete completed key={id}"))
                    }
                    WriteOp::Batch { .. } => {}
                }
            }
        }
    }
}

async fn connect_with_retry(config: &DatabaseConfig) -> SurrealRepository {
    let mut delay_ms = config.reconnect_initial_ms.max(1);
    let max_delay_ms = config.reconnect_max_ms.max(delay_ms);

    loop {
        match SurrealRepository::connect(config).await {
            Ok(repository) => {
                super::PERSISTENCE_SERVICE
                    .metrics
                    .connected
                    .store(true, Ordering::Relaxed);
                log::info(format_args!("surrealdb connected"));
                return repository;
            }
            Err(error) => {
                super::PERSISTENCE_SERVICE
                    .metrics
                    .connected
                    .store(false, Ordering::Relaxed);
                log::error(format_args!(
                    "surrealdb connect failed: {error}; retrying in {delay_ms}ms"
                ));
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                delay_ms = (delay_ms.saturating_mul(2)).min(max_delay_ms);
            }
        }
    }
}

async fn hydrate(repository: &SurrealRepository) {
    repository.define_tables().await;
    let records = surreal::hydrate(repository).await;
    hydrate_cache(records);
}
