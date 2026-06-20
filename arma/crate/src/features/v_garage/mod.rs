mod lifecycle;
mod query;
mod storage;

use forge_lib::{repositories::VGarageRepository, services::VGarageService};

#[derive(Clone)]
pub(crate) struct VGarageFeature<R> {
    service: VGarageService<R>,
}

impl<R> VGarageFeature<R>
where
    R: VGarageRepository,
{
    pub(crate) const fn new(service: VGarageService<R>) -> Self {
        Self { service }
    }
}
