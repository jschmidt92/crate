use forge_lib::{
    models::ServiceReceipt, repositories::BankRepository, services::MedicalService,
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

    pub(crate) fn record_respawn(&self, uid: &str) -> Result<ServiceReceipt, ServiceError> {
        self.service.record_respawn(uid)
    }

    pub(crate) fn full_heal(&self, uid: &str) -> Result<ServiceReceipt, ServiceError> {
        self.service.full_heal(uid)
    }
}
