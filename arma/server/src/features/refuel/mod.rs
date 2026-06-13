use forge_lib::{
    models::{FuelType, ServiceQuote, ServiceReceipt},
    repositories::BankRepository,
    services::FuelService,
    shared::ServiceError,
};

#[derive(Clone)]
pub(crate) struct FuelFeature<R> {
    service: FuelService<R>,
}

impl<R> FuelFeature<R>
where
    R: BankRepository,
{
    pub(crate) const fn new(service: FuelService<R>) -> Self {
        Self { service }
    }

    pub(crate) fn price(&self, fuel_type: FuelType) -> f64 {
        self.service.price(fuel_type)
    }

    pub(crate) fn quote(
        &self,
        liters: f64,
        fuel_type: FuelType,
    ) -> Result<ServiceQuote, ServiceError> {
        self.service.quote(liters, fuel_type)
    }

    pub(crate) fn complete(
        &self,
        uid: &str,
        plate: &str,
        liters: f64,
        fuel_type: FuelType,
    ) -> Result<ServiceReceipt, ServiceError> {
        self.service.complete(uid, plate, liters, fuel_type)
    }
}
