use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub uid: String,
    pub kind: NotificationKind,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
}

impl Notification {
    pub fn new(
        uid: impl Into<String>,
        kind: NotificationKind,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            uid: uid.into(),
            kind,
            title: title.into(),
            body: body.into(),
            created_at: Utc::now(),
            read_at: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationKind {
    OrganizationDisbanded,
    OrganizationInvite,
    OrganizationJoined,
    OrganizationKicked,
    OrganizationLeft,
    OrganizationPayday,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditRecord {
    pub id: Uuid,
    pub actor_uid: String,
    pub action: AuditAction,
    pub subject_id: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

impl AuditRecord {
    pub fn new(
        actor_uid: impl Into<String>,
        action: AuditAction,
        subject_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            actor_uid: actor_uid.into(),
            action,
            subject_id: subject_id.into(),
            message: message.into(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    LockerTransferCommitted,
    OrganizationCreated,
    OrganizationDisbanded,
    OrganizationInviteCreated,
    OrganizationInviteAccepted,
    OrganizationInviteDeclined,
    OrganizationMemberKicked,
    OrganizationMemberLeft,
    OrganizationPaydayIssued,
}
