use forge_lib::{
    models::{Money, ServiceReceipt},
    repositories::BankRepository,
    services::MedicalService,
    shared::ServiceError,
};

#[derive(Clone)]
pub(crate) struct MedicalFeature<R> {
    service: MedicalService<R>,
}

impl<R> MedicalFeature<R>
where
    R: BankRepository,
{
    pub(crate) const fn new(service: MedicalService<R>) -> Self {
        Self { service }
    }

    pub(crate) fn record_respawn(
        &self,
        uid: &str,
        respawn_price: Money,
    ) -> Result<ServiceReceipt, ServiceError> {
        self.service.record_respawn_with_fee(uid, respawn_price)
    }

    pub(crate) fn full_heal(
        &self,
        uid: &str,
        full_heal_price: Money,
    ) -> Result<ServiceReceipt, ServiceError> {
        self.service.full_heal_with_fee(uid, full_heal_price)
    }
}
