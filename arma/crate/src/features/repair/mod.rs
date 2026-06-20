use forge_lib::{
    models::{Money, ServiceQuote, ServiceReceipt},
    repositories::BankRepository,
    services::RepairService,
    shared::ServiceError,
};

#[derive(Clone)]
pub(crate) struct RepairFeature<R> {
    service: RepairService<R>,
}

impl<R> RepairFeature<R>
where
    R: BankRepository,
{
    pub(crate) const fn new(service: RepairService<R>) -> Self {
        Self { service }
    }

    pub(crate) fn quote(
        &self,
        damage: f64,
        full_repair_price: Money,
    ) -> Result<ServiceQuote, ServiceError> {
        self.service.quote_with_fee(damage, full_repair_price)
    }

    pub(crate) fn complete(
        &self,
        uid: &str,
        plate: &str,
        damage: f64,
        full_repair_price: Money,
    ) -> Result<ServiceReceipt, ServiceError> {
        self.service
            .complete_with_fee(uid, plate, damage, full_repair_price)
    }
}
