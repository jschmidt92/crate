use super::GarageFeature;
use forge_lib::{models::PlayerGarage, repositories::GarageRepository, shared::GarageError};

impl<R> GarageFeature<R>
where
    R: GarageRepository,
{
    pub(crate) fn save(&self, garage: PlayerGarage) -> Result<PlayerGarage, GarageError> {
        self.service.save(garage)
    }
}
