use serde::{Deserialize, Serialize};

use super::{
    ActorCreated, ActorDisconnected, OrganizationCreated, OrganizationDisbanded,
    OrganizationInviteAccepted, OrganizationInviteCreated, OrganizationInviteDeclined,
    OrganizationMemberKicked, OrganizationMemberLeft, OrganizationPaydayIssued,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomainEvent {
    ActorCreated(ActorCreated),
    ActorDisconnected(ActorDisconnected),
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
            Self::ActorDisconnected(_) => "actor.disconnected",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actor_disconnected_uses_expected_event_name() {
        let event = DomainEvent::ActorDisconnected(ActorDisconnected::new("76561198000000000"));

        assert_eq!(event.name(), "actor.disconnected");
    }
}
