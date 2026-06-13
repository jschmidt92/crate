use super::{OrganizationFeature, PaydayApplier};
use forge_lib::{
    events::EventPublisher,
    models::{
        DomainEvent, Organization, OrganizationCreated, OrganizationDisband, OrganizationDisbanded,
        OrganizationView, VGarage, VLocker,
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
    pub(crate) fn create_default_org_with_starting(
        &self,
        starting_bank: &str,
        virtual_garage: &VGarage,
        virtual_locker: &VLocker,
    ) -> Result<Organization, OrganizationError> {
        self.service
            .create_default_org_with_starting(starting_bank, virtual_garage, virtual_locker)
    }

    pub(crate) fn create_player_org(
        &self,
        id: &str,
        name: &str,
        ceo_uid: &str,
    ) -> Result<Organization, OrganizationError> {
        let organization = self.service.create_player_org(id, name, ceo_uid)?;
        self.events
            .publish(DomainEvent::OrganizationCreated(OrganizationCreated::new(
                OrganizationView::from(&organization),
                ceo_uid,
            )));
        Ok(organization)
    }

    pub(crate) fn disband_player_org(
        &self,
        organization_id: &str,
        ceo_uid: &str,
    ) -> Result<OrganizationDisband, OrganizationError> {
        let disband = self.service.disband_player_org(organization_id, ceo_uid)?;
        self.events.publish(DomainEvent::OrganizationDisbanded(
            OrganizationDisbanded::new(
                disband.disbanded.clone(),
                ceo_uid,
                disband.reassigned_uids.clone(),
            ),
        ));
        Ok(disband)
    }
}
