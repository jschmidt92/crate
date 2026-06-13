mod lifecycle;
mod query;
mod storage;

use forge_lib::{repositories::LockerRepository, services::LockerService};

#[derive(Clone)]
pub(crate) struct LockerFeature<R> {
    service: LockerService<R>,
}

impl<R> LockerFeature<R>
where
    R: LockerRepository,
{
    pub(crate) const fn new(service: LockerService<R>) -> Self {
        Self { service }
    }
}
