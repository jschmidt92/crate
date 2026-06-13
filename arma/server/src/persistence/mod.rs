mod model;
mod repository;
mod service;
mod surreal;

use crate::{config::DatabaseConfig, log};
use forge_lib::models::{
    Actor, Organization, OrganizationPaydayPlan, PlayerBankProfile, PlayerGarage, PlayerLocker,
    PlayerVGarage, PlayerVLocker,
};
use forge_lib::repositories::BankRepository;
use std::sync::LazyLock;

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

pub(super) fn enqueue(op: WriteOp) {
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

pub fn apply_payday_plan(
    plan: OrganizationPaydayPlan,
) -> Result<forge_lib::models::OrganizationPayday, forge_lib::shared::BankError> {
    let mut ops = Vec::new();
    let organization = ORGANIZATION_REPOSITORY
        .save_without_enqueue(plan.organization)
        .map_err(|error| forge_lib::shared::BankError::Repository(error.to_string()))?;

    ops.push(upsert_op("organization", &organization.id, &organization)?);

    for recipient_uid in &plan.recipients {
        let mut profile = BANK_REPOSITORY
            .find_by_uid(recipient_uid)
            .map_err(|error| forge_lib::shared::BankError::Repository(error.to_string()))?
            .unwrap_or_else(|| PlayerBankProfile::new(recipient_uid));
        profile.account.deposit(plan.amount);
        let profile = BANK_REPOSITORY.save_without_enqueue(profile)?;
        ops.push(upsert_op("bank", &profile.uid, &profile)?);
    }

    enqueue(WriteOp::Batch { ops });

    Ok(forge_lib::models::OrganizationPayday {
        organization: forge_lib::models::OrganizationView::from(&organization),
        amount: plan.amount.to_amount(),
        recipients: plan.recipients,
    })
}

fn upsert_op<T>(
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

pub(crate) struct DurableEventBackend;

impl forge_lib::events::DomainEventHandler for DurableEventBackend {
    fn name(&self) -> &'static str {
        "persistence.durable_event_backend"
    }

    fn handle(
        &self,
        event: &forge_lib::models::DomainEvent,
    ) -> Result<(), forge_lib::shared::EventError> {
        let mut ops = vec![upsert_event_op(event).map_err(|message| {
            forge_lib::shared::EventError::HandlerFailed {
                handler: self.name(),
                event: event.name(),
                message,
            }
        })?];

        append_event_side_effects(event, &mut ops).map_err(|message| {
            forge_lib::shared::EventError::HandlerFailed {
                handler: self.name(),
                event: event.name(),
                message,
            }
        })?;

        enqueue(WriteOp::Batch { ops });
        Ok(())
    }
}

fn upsert_event_op(event: &forge_lib::models::DomainEvent) -> Result<WriteOp, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let value = serde_json::json!({
        "id": id,
        "name": event.name(),
        "payload": event,
        "created_at": chrono::Utc::now(),
    });

    Ok(WriteOp::Upsert {
        table: "domain_event",
        id,
        value,
    })
}

fn append_event_side_effects(
    event: &forge_lib::models::DomainEvent,
    ops: &mut Vec<WriteOp>,
) -> Result<(), String> {
    use forge_lib::models::{
        AuditAction, AuditRecord, DomainEvent, Notification, NotificationKind,
    };

    match event {
        DomainEvent::ActorCreated(_) => {}
        DomainEvent::OrganizationCreated(event) => {
            push_audit(
                ops,
                AuditRecord::new(
                    &event.actor_uid,
                    AuditAction::OrganizationCreated,
                    &event.organization.id,
                    format!("created organization {}", event.organization.name),
                ),
            )?;
        }
        DomainEvent::OrganizationDisbanded(event) => {
            push_audit(
                ops,
                AuditRecord::new(
                    &event.actor_uid,
                    AuditAction::OrganizationDisbanded,
                    &event.organization.id,
                    format!("disbanded organization {}", event.organization.name),
                ),
            )?;
            for uid in &event.reassigned_uids {
                push_notification(
                    ops,
                    Notification::new(
                        uid,
                        NotificationKind::OrganizationDisbanded,
                        "Organization disbanded",
                        format!(
                            "{} was disbanded; you were moved to the default organization",
                            event.organization.name
                        ),
                    ),
                )?;
            }
        }
        DomainEvent::OrganizationInviteCreated(event) => {
            push_audit(
                ops,
                AuditRecord::new(
                    &event.invite.inviter_uid,
                    AuditAction::OrganizationInviteCreated,
                    event.invite.id.to_string(),
                    format!(
                        "invited {} to organization {}",
                        event.invite.invitee_uid, event.invite.organization_id
                    ),
                ),
            )?;
            push_notification(
                ops,
                Notification::new(
                    &event.invite.invitee_uid,
                    NotificationKind::OrganizationInvite,
                    "Organization invite",
                    format!(
                        "You were invited to organization {}",
                        event.invite.organization_id
                    ),
                ),
            )?;
        }
        DomainEvent::OrganizationInviteAccepted(event) => {
            push_audit(
                ops,
                AuditRecord::new(
                    &event.invite.invitee_uid,
                    AuditAction::OrganizationInviteAccepted,
                    event.invite.id.to_string(),
                    format!("accepted organization invite {}", event.invite.id),
                ),
            )?;
            push_notification(
                ops,
                Notification::new(
                    &event.invite.inviter_uid,
                    NotificationKind::OrganizationJoined,
                    "Invite accepted",
                    format!(
                        "{} joined {}",
                        event.invite.invitee_uid, event.invite.organization_id
                    ),
                ),
            )?;
        }
        DomainEvent::OrganizationInviteDeclined(event) => {
            push_audit(
                ops,
                AuditRecord::new(
                    &event.invite.invitee_uid,
                    AuditAction::OrganizationInviteDeclined,
                    event.invite.id.to_string(),
                    format!("declined organization invite {}", event.invite.id),
                ),
            )?;
        }
        DomainEvent::OrganizationMemberLeft(event) => {
            push_audit(
                ops,
                AuditRecord::new(
                    &event.uid,
                    AuditAction::OrganizationMemberLeft,
                    &event.organization.id,
                    format!("left organization {}", event.organization.name),
                ),
            )?;
            push_notification(
                ops,
                Notification::new(
                    &event.uid,
                    NotificationKind::OrganizationLeft,
                    "Organization left",
                    format!(
                        "You left {}; you were moved to the default organization",
                        event.organization.name
                    ),
                ),
            )?;
        }
        DomainEvent::OrganizationMemberKicked(event) => {
            push_audit(
                ops,
                AuditRecord::new(
                    &event.actor_uid,
                    AuditAction::OrganizationMemberKicked,
                    &event.organization.id,
                    format!(
                        "kicked {} from organization {}",
                        event.kicked_uid, event.organization.name
                    ),
                ),
            )?;
            push_notification(
                ops,
                Notification::new(
                    &event.kicked_uid,
                    NotificationKind::OrganizationKicked,
                    "Removed from organization",
                    format!(
                        "You were removed from {}; you were moved to the default organization",
                        event.organization.name
                    ),
                ),
            )?;
        }
        DomainEvent::OrganizationPaydayIssued(event) => {
            push_audit(
                ops,
                AuditRecord::new(
                    &event.issuer_uid,
                    AuditAction::OrganizationPaydayIssued,
                    &event.organization.id,
                    format!(
                        "issued payday of {} to {} recipients",
                        event.amount.as_str(),
                        event.recipients.len()
                    ),
                ),
            )?;
            for recipient_uid in &event.recipients {
                push_notification(
                    ops,
                    Notification::new(
                        recipient_uid,
                        NotificationKind::OrganizationPayday,
                        "Payday received",
                        format!(
                            "{} paid you {}",
                            event.organization.name,
                            event.amount.as_str()
                        ),
                    ),
                )?;
            }
        }
    }

    Ok(())
}

fn push_audit(
    ops: &mut Vec<WriteOp>,
    record: forge_lib::models::AuditRecord,
) -> Result<(), String> {
    let value = serde_json::to_value(&record).map_err(|error| error.to_string())?;
    ops.push(WriteOp::Upsert {
        table: "audit_record",
        id: record.id.to_string(),
        value,
    });
    Ok(())
}

fn push_notification(
    ops: &mut Vec<WriteOp>,
    notification: forge_lib::models::Notification,
) -> Result<(), String> {
    let value = serde_json::to_value(&notification).map_err(|error| error.to_string())?;
    ops.push(WriteOp::Upsert {
        table: "notification",
        id: notification.id.to_string(),
        value,
    });
    Ok(())
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
