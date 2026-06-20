use super::{NOTIFICATION_REPOSITORY, enqueue, model::WriteOp};

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
        DomainEvent::ActorCreated(_) | DomainEvent::ActorDisconnected(_) => {}
        DomainEvent::LockerTransferCommitted(event) => {
            push_audit(
                ops,
                AuditRecord::new(
                    &event.uid,
                    AuditAction::LockerTransferCommitted,
                    &event.uid,
                    format!(
                        "committed locker transfer with {} item classes and {} total items",
                        event.distinct_items, event.total_quantity
                    ),
                ),
            )?;
        }
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
    let notification = NOTIFICATION_REPOSITORY
        .save_without_enqueue(notification)
        .map_err(|error| error.to_string())?;
    let value = serde_json::to_value(&notification).map_err(|error| error.to_string())?;
    ops.push(WriteOp::Upsert {
        table: "notification",
        id: notification.id.to_string(),
        value,
    });
    Ok(())
}
