mod lifecycle;
mod query;
mod storage;

use forge_lib::{repositories::VLockerRepository, services::VLockerService};

#[derive(Clone)]
pub(crate) struct VLockerFeature<R> {
    service: VLockerService<R>,
}

impl<R> VLockerFeature<R>
where
    R: VLockerRepository,
{
    pub(crate) const fn new(service: VLockerService<R>) -> Self {
        Self { service }
    }
}
