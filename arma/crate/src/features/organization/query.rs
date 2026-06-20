use super::{OrganizationFeature, PaydayApplier};
use forge_lib::{
    events::EventPublisher, models::Organization, repositories::OrganizationRepository,
    shared::OrganizationError,
};

impl<R, E, P> OrganizationFeature<R, E, P>
where
    R: OrganizationRepository,
    E: EventPublisher,
    P: PaydayApplier,
{
    pub(crate) fn get(&self, id: &str) -> Result<Option<Organization>, OrganizationError> {
        self.service.get(id)
    }

    pub(crate) fn get_by_member_uid(
        &self,
        uid: &str,
    ) -> Result<Option<Organization>, OrganizationError> {
        self.service.get_by_member_uid(uid)
    }
}
