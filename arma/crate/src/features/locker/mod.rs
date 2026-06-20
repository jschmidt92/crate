mod lifecycle;
mod query;
mod storage;
mod transfer;

use forge_lib::{events::EventPublisher, repositories::LockerRepository, services::LockerService};

#[derive(Clone)]
pub(crate) struct LockerFeature<R, E> {
    service: LockerService<R>,
    events: E,
}

impl<R, E> LockerFeature<R, E>
where
    R: LockerRepository,
    E: EventPublisher,
{
    pub(crate) const fn new(service: LockerService<R>, events: E) -> Self {
        Self { service, events }
    }
}
