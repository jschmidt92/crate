use serde::{Deserialize, Serialize};

use super::{MoneyAmount, OrganizationInvite, OrganizationView};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationCreated {
    pub organization: OrganizationView,
    pub actor_uid: String,
}

impl OrganizationCreated {
    pub fn new(organization: OrganizationView, actor_uid: impl Into<String>) -> Self {
        Self {
            organization,
            actor_uid: actor_uid.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationDisbanded {
    pub organization: OrganizationView,
    pub actor_uid: String,
    pub reassigned_uids: Vec<String>,
}

impl OrganizationDisbanded {
    pub fn new(
        organization: OrganizationView,
        actor_uid: impl Into<String>,
        reassigned_uids: Vec<String>,
    ) -> Self {
        Self {
            organization,
            actor_uid: actor_uid.into(),
            reassigned_uids,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInviteCreated {
    pub invite: OrganizationInvite,
}

impl OrganizationInviteCreated {
    pub const fn new(invite: OrganizationInvite) -> Self {
        Self { invite }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInviteAccepted {
    pub invite: OrganizationInvite,
}

impl OrganizationInviteAccepted {
    pub const fn new(invite: OrganizationInvite) -> Self {
        Self { invite }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganizationInviteDeclined {
    pub invite: OrganizationInvite,
}

impl OrganizationInviteDeclined {
    pub const fn new(invite: OrganizationInvite) -> Self {
        Self { invite }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationMemberLeft {
    pub organization: OrganizationView,
    pub default_organization: OrganizationView,
    pub uid: String,
}

impl OrganizationMemberLeft {
    pub fn new(
        organization: OrganizationView,
        default_organization: OrganizationView,
        uid: impl Into<String>,
    ) -> Self {
        Self {
            organization,
            default_organization,
            uid: uid.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationMemberKicked {
    pub organization: OrganizationView,
    pub default_organization: OrganizationView,
    pub actor_uid: String,
    pub kicked_uid: String,
}

impl OrganizationMemberKicked {
    pub fn new(
        organization: OrganizationView,
        default_organization: OrganizationView,
        actor_uid: impl Into<String>,
        kicked_uid: impl Into<String>,
    ) -> Self {
        Self {
            organization,
            default_organization,
            actor_uid: actor_uid.into(),
            kicked_uid: kicked_uid.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationPaydayIssued {
    pub organization: OrganizationView,
    pub issuer_uid: String,
    pub amount: MoneyAmount,
    pub recipients: Vec<String>,
}

impl OrganizationPaydayIssued {
    pub fn new(
        organization: OrganizationView,
        issuer_uid: impl Into<String>,
        amount: MoneyAmount,
        recipients: Vec<String>,
    ) -> Self {
        Self {
            organization,
            issuer_uid: issuer_uid.into(),
            amount,
            recipients,
        }
    }
}
