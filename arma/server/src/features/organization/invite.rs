use super::{OrganizationFeature, PaydayApplier};
use forge_lib::{
    events::EventPublisher,
    models::{
        DomainEvent, Organization, OrganizationInvite, OrganizationInviteAccepted,
        OrganizationInviteCreated, OrganizationInviteDeclined,
    },
    repositories::OrganizationRepository,
    shared::OrganizationError,
};

impl<R, E, P> OrganizationFeature<R, E, P>
where
    R: OrganizationRepository,
    E: EventPublisher,
    P: PaydayApplier,
{
    pub(crate) fn create_invite(
        &self,
        inviter_uid: &str,
        organization_id: &str,
        invitee_uid: &str,
    ) -> Result<OrganizationInvite, OrganizationError> {
        let invite = self
            .service
            .create_invite(inviter_uid, organization_id, invitee_uid)?;
        self.events.publish(DomainEvent::OrganizationInviteCreated(
            OrganizationInviteCreated::new(invite.clone()),
        ));
        Ok(invite)
    }

    pub(crate) fn accept_invite(
        &self,
        invitee_uid: &str,
        invite_id: &str,
    ) -> Result<Organization, OrganizationError> {
        let (organization, invite) = self.service.accept_invite(invitee_uid, invite_id)?;
        self.events.publish(DomainEvent::OrganizationInviteAccepted(
            OrganizationInviteAccepted::new(invite),
        ));
        Ok(organization)
    }

    pub(crate) fn decline_invite(
        &self,
        invitee_uid: &str,
        invite_id: &str,
    ) -> Result<OrganizationInvite, OrganizationError> {
        let invite = self.service.decline_invite(invitee_uid, invite_id)?;
        self.events.publish(DomainEvent::OrganizationInviteDeclined(
            OrganizationInviteDeclined::new(invite.clone()),
        ));
        Ok(invite)
    }
}
