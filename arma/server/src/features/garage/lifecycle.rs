use super::GarageFeature;
use forge_lib::{models::PlayerGarage, repositories::GarageRepository, shared::GarageError};

impl<R> GarageFeature<R>
where
    R: GarageRepository,
{
    pub(crate) fn init(&self, uid: &str) -> Result<PlayerGarage, GarageError> {
        self.service.create_actor_garage(uid)
    }

    pub(crate) fn disconnect(&self, uid: &str) -> Result<(), GarageError> {
        self.service.disconnect(uid)
    }

    pub(crate) fn delete(&self, uid: &str) -> Result<(), GarageError> {
        self.service.delete(uid)
    }
}
