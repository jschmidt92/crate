use super::model::WriteOp;
use crate::{config::DatabaseConfig, log};
use forge_lib::models::{
    Actor, Notification, Organization, OrganizationInvite, PlayerBankProfile, PlayerGarage,
    PlayerLocker, PlayerVGarage, PlayerVLocker,
};
use surrealdb::{Surreal, engine::remote::ws::Client, engine::remote::ws::Ws, opt::auth::Root};

const TABLES: &[&str] = &[
    "actor",
    "bank",
    "garage",
    "locker",
    "notification",
    "organization",
    "organization_invite",
    "v_garage",
    "v_locker",
    "audit_record",
    "domain_event",
];

pub struct HydratedRecords {
    pub actors: Vec<Actor>,
    pub bank_profiles: Vec<PlayerBankProfile>,
    pub garages: Vec<PlayerGarage>,
    pub lockers: Vec<PlayerLocker>,
    pub notifications: Vec<Notification>,
    pub organizations: Vec<Organization>,
    pub organization_invites: Vec<OrganizationInvite>,
    pub v_garages: Vec<PlayerVGarage>,
    pub v_lockers: Vec<PlayerVLocker>,
}

pub struct SurrealRepository {
    db: Surreal<Client>,
}

impl SurrealRepository {
    pub async fn connect(config: &DatabaseConfig) -> surrealdb::Result<Self> {
        let db = Surreal::new::<Ws>(&config.endpoint).await?;
        db.signin(Root {
            username: config.username.clone(),
            password: config.password.clone(),
        })
        .await?;
        db.use_ns(&config.namespace)
            .use_db(&config.database)
            .await?;
        Ok(Self { db })
    }

    pub async fn define_tables(&self) {
        let mut sql = String::new();
        for table in TABLES {
            sql.push_str(&format!("DEFINE TABLE IF NOT EXISTS {table} SCHEMALESS;\n"));
        }
        match self.db.query(sql).await {
            Ok(_) => log::info(format_args!(
                "surrealdb tables verified ({} tables)",
                TABLES.len()
            )),
            Err(error) => log::error(format_args!("surrealdb define tables failed: {error}")),
        }
    }

    pub async fn apply(&self, op: &WriteOp) -> surrealdb::Result<()> {
        match op {
            WriteOp::Upsert { table, id, value } => {
                let _: Option<serde_json::Value> = self
                    .db
                    .upsert((*table, id.as_str()))
                    .content(value.clone())
                    .await?;
            }
            WriteOp::Delete { table, id } => {
                let _: Option<serde_json::Value> = self.db.delete((*table, id.as_str())).await?;
            }
            WriteOp::Batch { ops } => {
                self.apply_batch(ops).await?;
            }
        }

        Ok(())
    }

    async fn apply_batch(&self, ops: &[WriteOp]) -> surrealdb::Result<()> {
        let mut sql = String::from("BEGIN TRANSACTION;\n");
        for (index, op) in ops.iter().enumerate() {
            match op {
                WriteOp::Upsert { .. } => {
                    sql.push_str(&format!(
                        "UPSERT type::record($table{index}, $id{index}) CONTENT $value{index};\n"
                    ));
                }
                WriteOp::Delete { .. } => {
                    sql.push_str(&format!(
                        "DELETE type::record($table{index}, $id{index});\n"
                    ));
                }
                WriteOp::Batch { .. } => {}
            }
        }
        sql.push_str("COMMIT TRANSACTION;");

        let mut query = self.db.query(sql);
        for (index, op) in ops.iter().enumerate() {
            match op {
                WriteOp::Upsert { table, id, value } => {
                    query = query
                        .bind((format!("table{index}"), *table))
                        .bind((format!("id{index}"), id.clone()))
                        .bind((format!("value{index}"), value.clone()));
                }
                WriteOp::Delete { table, id } => {
                    query = query
                        .bind((format!("table{index}"), *table))
                        .bind((format!("id{index}"), id.clone()));
                }
                WriteOp::Batch { .. } => {}
            }
        }

        query.await?;
        Ok(())
    }
}

pub async fn hydrate(repository: &SurrealRepository) -> HydratedRecords {
    HydratedRecords {
        actors: select_table(&repository.db, "actor", "actor").await,
        bank_profiles: select_table(&repository.db, "bank", "bank").await,
        garages: select_table(&repository.db, "garage", "garage").await,
        lockers: select_table(&repository.db, "locker", "locker").await,
        notifications: select_table(&repository.db, "notification", "notification").await,
        organizations: select_table(&repository.db, "organization", "organization").await,
        organization_invites: select_table(
            &repository.db,
            "organization_invite",
            "organization invite",
        )
        .await,
        v_garages: select_table(&repository.db, "v_garage", "virtual garage").await,
        v_lockers: select_table(&repository.db, "v_locker", "virtual locker").await,
    }
}

async fn select_table<T>(db: &Surreal<Client>, table: &str, label: &str) -> Vec<T>
where
    T: serde::de::DeserializeOwned,
{
    let records: surrealdb::Result<Vec<serde_json::Value>> = db.select(table).await;
    match records {
        Ok(records) => {
            let mut items = Vec::with_capacity(records.len());
            for value in records {
                match serde_json::from_value::<T>(value) {
                    Ok(record) => items.push(record),
                    Err(error) => {
                        log::warn(format_args!(
                            "surrealdb {label} hydrate: skipped malformed record: {error}"
                        ));
                    }
                }
            }
            if items.is_empty() {
                log::info(format_args!("surrealdb {label}: no existing records"));
            } else {
                log::info(format_args!(
                    "surrealdb {label}: hydrated {} record(s)",
                    items.len()
                ));
            }
            items
        }
        Err(error) => {
            let msg = error.to_string();
            if msg.contains("does not exist") {
                log::warn(format_args!(
                    "surrealdb {label}: table not found, starting empty"
                ));
            } else {
                log::error(format_args!("surrealdb {label} hydrate failed: {error}"));
            }
            Vec::new()
        }
    }
}
