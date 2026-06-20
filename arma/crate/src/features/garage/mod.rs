mod lifecycle;
mod query;
mod storage;

use forge_lib::{repositories::GarageRepository, services::GarageService};

#[derive(Clone)]
pub(crate) struct GarageFeature<R> {
    service: GarageService<R>,
}

impl<R> GarageFeature<R>
where
    R: GarageRepository,
{
    pub(crate) const fn new(service: GarageService<R>) -> Self {
        Self { service }
    }
}
