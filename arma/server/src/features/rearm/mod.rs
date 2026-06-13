use forge_lib::{
    models::{Money, ServiceQuote, ServiceReceipt},
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

    pub(crate) fn quote(
        &self,
        units: u32,
        unit_price: Money,
    ) -> Result<ServiceQuote, ServiceError> {
        self.service.quote_with_fee(units, unit_price)
    }

    pub(crate) fn complete(
        &self,
        uid: &str,
        plate: &str,
        units: u32,
        unit_price: Money,
    ) -> Result<ServiceReceipt, ServiceError> {
        self.service
            .complete_with_fee(uid, plate, units, unit_price)
    }
}
