use super::{OrganizationFeature, PaydayApplier};
use forge_lib::{
    events::EventPublisher,
    models::{
        DomainEvent, Organization, OrganizationMemberKicked, OrganizationMemberLeft,
        OrganizationMemberTransfer,
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
    pub(crate) fn leave_member(
        &self,
        organization_id: &str,
        uid: &str,
    ) -> Result<OrganizationMemberTransfer, OrganizationError> {
        let transfer = self.service.leave_member(organization_id, uid)?;
        self.events.publish(DomainEvent::OrganizationMemberLeft(
            OrganizationMemberLeft::new(
                transfer.organization.clone(),
                transfer.default_organization.clone(),
                uid,
            ),
        ));
        Ok(transfer)
    }

    pub(crate) fn kick_member(
        &self,
        organization_id: &str,
        actor_uid: &str,
        kicked_uid: &str,
    ) -> Result<OrganizationMemberTransfer, OrganizationError> {
        let transfer = self
            .service
            .kick_member(organization_id, actor_uid, kicked_uid)?;
        self.events.publish(DomainEvent::OrganizationMemberKicked(
            OrganizationMemberKicked::new(
                transfer.organization.clone(),
                transfer.default_organization.clone(),
                actor_uid,
                kicked_uid,
            ),
        ));
        Ok(transfer)
    }

    pub(crate) fn add_member(
        &self,
        organization_id: &str,
        uid: &str,
    ) -> Result<Organization, OrganizationError> {
        self.service.add_member(organization_id, uid)
    }
}
