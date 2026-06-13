use serde::{Deserialize, Serialize};

use super::{
    ActorCreated, OrganizationCreated, OrganizationDisbanded, OrganizationInviteAccepted,
    OrganizationInviteCreated, OrganizationInviteDeclined, OrganizationMemberKicked,
    OrganizationMemberLeft, OrganizationPaydayIssued,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomainEvent {
    ActorCreated(ActorCreated),
    OrganizationCreated(OrganizationCreated),
    OrganizationDisbanded(OrganizationDisbanded),
    OrganizationInviteCreated(OrganizationInviteCreated),
    OrganizationInviteAccepted(OrganizationInviteAccepted),
    OrganizationInviteDeclined(OrganizationInviteDeclined),
    OrganizationMemberLeft(OrganizationMemberLeft),
    OrganizationMemberKicked(OrganizationMemberKicked),
    OrganizationPaydayIssued(OrganizationPaydayIssued),
}

impl DomainEvent {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::ActorCreated(_) => "actor.created",
            Self::OrganizationCreated(_) => "organization.created",
            Self::OrganizationDisbanded(_) => "organization.disbanded",
            Self::OrganizationInviteCreated(_) => "organization.invite_created",
            Self::OrganizationInviteAccepted(_) => "organization.invite_accepted",
            Self::OrganizationInviteDeclined(_) => "organization.invite_declined",
            Self::OrganizationMemberLeft(_) => "organization.member_left",
            Self::OrganizationMemberKicked(_) => "organization.member_kicked",
            Self::OrganizationPaydayIssued(_) => "organization.payday_issued",
        }
    }
}
