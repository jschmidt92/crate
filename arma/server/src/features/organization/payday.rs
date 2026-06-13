use super::{OrganizationFeature, OrganizationPaydayError};
use forge_lib::{
    events::EventPublisher,
    models::{DomainEvent, OrganizationPayday, OrganizationPaydayIssued, OrganizationPaydayPlan},
    repositories::OrganizationRepository,
    shared::BankError,
};

pub(crate) trait PaydayApplier: Send + Sync {
    fn apply(&self, plan: OrganizationPaydayPlan) -> Result<OrganizationPayday, BankError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct PersistencePaydayApplier;

impl PaydayApplier for PersistencePaydayApplier {
    fn apply(&self, plan: OrganizationPaydayPlan) -> Result<OrganizationPayday, BankError> {
        crate::persistence::apply_payday_plan(plan)
    }
}

impl<R, E, P> OrganizationFeature<R, E, P>
where
    R: OrganizationRepository,
    E: EventPublisher,
    P: PaydayApplier,
{
    pub(crate) fn issue_payday(
        &self,
        uid: &str,
        organization_id: &str,
        amount: &str,
        in_default_ceo_slot: bool,
    ) -> Result<OrganizationPayday, OrganizationPaydayError> {
        let plan =
            self.service
                .prepare_payday(uid, organization_id, amount, in_default_ceo_slot)?;
        let payday = self.payday_applier.apply(plan)?;
        self.events.publish(DomainEvent::OrganizationPaydayIssued(
            OrganizationPaydayIssued::new(
                payday.organization.clone(),
                uid,
                payday.amount.clone(),
                payday.recipients.clone(),
            ),
        ));
        Ok(payday)
    }
}
