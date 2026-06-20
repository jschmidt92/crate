use super::VGarageFeature;
use forge_lib::{models::PlayerVGarage, repositories::VGarageRepository, shared::VGarageError};

impl<R> VGarageFeature<R>
where
    R: VGarageRepository,
{
    pub(crate) fn save(&self, garage: PlayerVGarage) -> Result<PlayerVGarage, VGarageError> {
        self.service.save(garage)
    }
}
