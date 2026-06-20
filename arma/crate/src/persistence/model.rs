use std::{
    fmt,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

pub enum WriteOp {
    Upsert {
        table: &'static str,
        id: String,
        value: serde_json::Value,
    },
    Delete {
        table: &'static str,
        id: String,
    },
    Batch {
        ops: Vec<WriteOp>,
    },
}

#[derive(Debug, Default)]
pub struct PersistenceMetrics {
    pub enabled: AtomicBool,
    pub connected: AtomicBool,
    pub ready: AtomicBool,
    pub queued: AtomicUsize,
    pub dropped: AtomicUsize,
}

impl PersistenceMetrics {
    pub fn status(&self) -> PersistenceStatus {
        PersistenceStatus {
            enabled: self.enabled.load(Ordering::Relaxed),
            connected: self.connected.load(Ordering::Relaxed),
            ready: self.ready.load(Ordering::Relaxed),
            queued: self.queued.load(Ordering::Relaxed),
            dropped: self.dropped.load(Ordering::Relaxed),
        }
    }
}

pub struct PersistenceStatus {
    enabled: bool,
    connected: bool,
    ready: bool,
    queued: usize,
    dropped: usize,
}

impl fmt::Display for PersistenceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.enabled {
            return f.write_str("disabled");
        }

        let state = if self.ready {
            "ready"
        } else if self.connected {
            "hydrating"
        } else {
            "disconnected"
        };

        write!(
            f,
            "{state}; queued={}; dropped={}",
            self.queued, self.dropped
        )
    }
}
