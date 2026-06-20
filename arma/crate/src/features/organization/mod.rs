mod create;
mod invite;
mod membership;
mod payday;
mod query;

use forge_lib::{
    events::EventPublisher,
    repositories::OrganizationRepository,
    services::OrganizationService,
    shared::{BankError, OrganizationError},
};

pub(crate) use payday::{PaydayApplier, PersistencePaydayApplier};

#[derive(Debug, Clone)]
pub(crate) enum OrganizationPaydayError {
    Organization(OrganizationError),
    Bank(BankError),
}

impl std::fmt::Display for OrganizationPaydayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Organization(error) => error.fmt(f),
            Self::Bank(error) => error.fmt(f),
        }
    }
}

impl From<OrganizationError> for OrganizationPaydayError {
    fn from(value: OrganizationError) -> Self {
        Self::Organization(value)
    }
}

impl From<BankError> for OrganizationPaydayError {
    fn from(value: BankError) -> Self {
        Self::Bank(value)
    }
}

#[derive(Clone)]
pub(crate) struct OrganizationFeature<R, E, P> {
    service: OrganizationService<R>,
    events: E,
    payday_applier: P,
}

impl<R, E, P> OrganizationFeature<R, E, P>
where
    R: OrganizationRepository,
    E: EventPublisher,
    P: PaydayApplier,
{
    pub(crate) const fn new(service: OrganizationService<R>, events: E, payday_applier: P) -> Self {
        Self {
            service,
            events,
            payday_applier,
        }
    }
}
