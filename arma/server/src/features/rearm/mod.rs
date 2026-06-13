use forge_lib::{
    models::{ServiceQuote, ServiceReceipt},
    repositories::BankRepository,
    services::RearmService,
    shared::ServiceError,
};

#[derive(Clone)]
pub(crate) struct RearmFeature<R> {
    service: RearmService<R>,
}

impl<R> RearmFeature<R>
where
    R: BankRepository,
{
    pub(crate) const fn new(service: RearmService<R>) -> Self {
        Self { service }
    }

    pub(crate) fn quote(&self, units: u32) -> Result<ServiceQuote, ServiceError> {
        self.service.quote(units)
    }

    pub(crate) fn complete(
        &self,
        uid: &str,
        plate: &str,
        units: u32,
    ) -> Result<ServiceReceipt, ServiceError> {
        self.service.complete(uid, plate, units)
    }
}
