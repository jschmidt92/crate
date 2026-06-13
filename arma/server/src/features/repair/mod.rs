use forge_lib::{
    models::{ServiceQuote, ServiceReceipt},
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

    pub(crate) fn quote(&self, damage: f64) -> Result<ServiceQuote, ServiceError> {
        self.service.quote(damage)
    }

    pub(crate) fn complete(
        &self,
        uid: &str,
        plate: &str,
        damage: f64,
    ) -> Result<ServiceReceipt, ServiceError> {
        self.service.complete(uid, plate, damage)
    }
}
