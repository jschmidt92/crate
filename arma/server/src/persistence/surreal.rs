use super::model::WriteOp;
use crate::{config::DatabaseConfig, log};
use forge_lib::models::{
    Actor, Organization, PlayerBankProfile, PlayerGarage, PlayerLocker, PlayerVGarage,
    PlayerVLocker,
};
use surrealdb::{Surreal, engine::remote::ws::Client, engine::remote::ws::Ws, opt::auth::Root};

pub struct HydratedRecords {
    pub actors: Vec<Actor>,
    pub bank_profiles: Vec<PlayerBankProfile>,
    pub garages: Vec<PlayerGarage>,
    pub lockers: Vec<PlayerLocker>,
    pub organizations: Vec<Organization>,
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
        }

        Ok(())
    }
}

pub async fn hydrate(repository: &SurrealRepository) -> HydratedRecords {
    HydratedRecords {
        actors: select_table(&repository.db, "actor", "actor").await,
        bank_profiles: select_table(&repository.db, "bank", "bank").await,
        garages: select_table(&repository.db, "garage", "garage").await,
        lockers: select_table(&repository.db, "locker", "locker").await,
        organizations: select_table(&repository.db, "organization", "organization").await,
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
        Ok(records) => records
            .into_iter()
            .filter_map(|value| match serde_json::from_value::<T>(value) {
                Ok(record) => Some(record),
                Err(error) => {
                    log::error(format_args!(
                        "surrealdb {label} hydrate decode failed: {error}"
                    ));
                    None
                }
            })
            .collect(),
        Err(error) => {
            log::error(format_args!("surrealdb {label} hydrate failed: {error}"));
            Vec::new()
        }
    }
}
