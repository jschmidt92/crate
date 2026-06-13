use super::GarageFeature;
use forge_lib::{models::PlayerGarage, repositories::GarageRepository, shared::GarageError};

impl<R> GarageFeature<R>
where
    R: GarageRepository,
{
    pub(crate) fn get(&self, uid: &str) -> Result<Option<PlayerGarage>, GarageError> {
        self.service.get(uid)
    }
}
