mod durable_events;
mod model;
mod payday;
mod repository;
mod service;
mod surreal;

use crate::{config::DatabaseConfig, log};
use forge_lib::models::{
    Actor, Organization, PlayerBankProfile, PlayerGarage, PlayerLocker, PlayerVGarage,
    PlayerVLocker,
};
use std::sync::LazyLock;

pub(crate) use durable_events::DurableEventBackend;
pub use payday::apply_payday_plan;
pub use repository::{
    CachedActorRepository, CachedBankRepository, CachedGarageRepository, CachedLockerRepository,
    CachedOrganizationRepository, CachedVGarageRepository, CachedVLockerRepository,
};

use model::WriteOp;
use repository::{
    cache_actor, cache_bank_profile, cache_garage, cache_locker, cache_organization,
    cache_organization_invite, cache_v_garage, cache_v_locker,
};
use service::PersistenceService;

static ACTOR_REPOSITORY: LazyLock<CachedActorRepository> =
    LazyLock::new(CachedActorRepository::new);
static BANK_REPOSITORY: LazyLock<CachedBankRepository> = LazyLock::new(CachedBankRepository::new);
static GARAGE_REPOSITORY: LazyLock<CachedGarageRepository> =
    LazyLock::new(CachedGarageRepository::new);
static LOCKER_REPOSITORY: LazyLock<CachedLockerRepository> =
    LazyLock::new(CachedLockerRepository::new);
static ORGANIZATION_REPOSITORY: LazyLock<CachedOrganizationRepository> =
    LazyLock::new(CachedOrganizationRepository::new);
static V_GARAGE_REPOSITORY: LazyLock<CachedVGarageRepository> =
    LazyLock::new(CachedVGarageRepository::new);
static V_LOCKER_REPOSITORY: LazyLock<CachedVLockerRepository> =
    LazyLock::new(CachedVLockerRepository::new);
static PERSISTENCE_SERVICE: LazyLock<PersistenceService> = LazyLock::new(PersistenceService::new);

pub fn init(config: DatabaseConfig) {
    let _ = &*ACTOR_REPOSITORY;
    let _ = &*BANK_REPOSITORY;
    let _ = &*GARAGE_REPOSITORY;
    let _ = &*LOCKER_REPOSITORY;
    let _ = &*ORGANIZATION_REPOSITORY;
    let _ = &*V_GARAGE_REPOSITORY;
    let _ = &*V_LOCKER_REPOSITORY;

    if !config.enabled {
        log::info(format_args!(
            "surrealdb persistence disabled; using in-memory repositories"
        ));
        return;
    }

    PERSISTENCE_SERVICE.start(config);
}

pub fn status() -> String {
    PERSISTENCE_SERVICE.status().to_string()
}

pub fn actor_repository() -> CachedActorRepository {
    ACTOR_REPOSITORY.clone()
}

pub fn bank_repository() -> CachedBankRepository {
    BANK_REPOSITORY.clone()
}

pub fn garage_repository() -> CachedGarageRepository {
    GARAGE_REPOSITORY.clone()
}

pub fn locker_repository() -> CachedLockerRepository {
    LOCKER_REPOSITORY.clone()
}

pub fn organization_repository() -> CachedOrganizationRepository {
    ORGANIZATION_REPOSITORY.clone()
}

pub fn v_garage_repository() -> CachedVGarageRepository {
    V_GARAGE_REPOSITORY.clone()
}

pub fn v_locker_repository() -> CachedVLockerRepository {
    V_LOCKER_REPOSITORY.clone()
}

pub(crate) fn enqueue(op: WriteOp) {
    PERSISTENCE_SERVICE.enqueue(op);
}

pub(super) fn hydrate_cache(records: surreal::HydratedRecords) {
    for actor in records.actors {
        cache_actor(&ACTOR_REPOSITORY, actor);
    }

    for profile in records.bank_profiles {
        cache_bank_profile(&BANK_REPOSITORY, profile);
    }

    for garage in records.garages {
        cache_garage(&GARAGE_REPOSITORY, garage);
    }

    for locker in records.lockers {
        cache_locker(&LOCKER_REPOSITORY, locker);
    }

    for organization in records.organizations {
        cache_organization(&ORGANIZATION_REPOSITORY, organization);
    }

    for invite in records.organization_invites {
        cache_organization_invite(&ORGANIZATION_REPOSITORY, invite);
    }

    for garage in records.v_garages {
        cache_v_garage(&V_GARAGE_REPOSITORY, garage);
    }

    for locker in records.v_lockers {
        cache_v_locker(&V_LOCKER_REPOSITORY, locker);
    }
}

pub(super) fn enqueue_upsert<T>(table: &'static str, id: &str, value: &T)
where
    T: serde::Serialize,
{
    let Ok(value) = serde_json::to_value(value) else {
        log::error(format_args!(
            "failed to serialize {table}:{id} for surrealdb"
        ));
        return;
    };
    enqueue(WriteOp::Upsert {
        table,
        id: id.to_string(),
        value,
    });
}

pub(super) fn enqueue_delete(table: &'static str, id: &str) {
    enqueue(WriteOp::Delete {
        table,
        id: id.to_string(),
    });
}

pub(crate) fn upsert_op<T>(
    table: &'static str,
    id: &str,
    value: &T,
) -> Result<WriteOp, forge_lib::shared::BankError>
where
    T: serde::Serialize,
{
    let value = serde_json::to_value(value).map_err(|error| {
        forge_lib::shared::BankError::Repository(format!(
            "failed to serialize {table}:{id}: {error}"
        ))
    })?;

    Ok(WriteOp::Upsert {
        table,
        id: id.to_string(),
        value,
    })
}

#[allow(dead_code)]
fn _type_guards(
    _: Actor,
    _: PlayerBankProfile,
    _: PlayerGarage,
    _: PlayerLocker,
    _: Organization,
    _: PlayerVGarage,
    _: PlayerVLocker,
) {
}
