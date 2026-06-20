use super::VGarageFeature;
use forge_lib::{models::PlayerVGarage, repositories::VGarageRepository, shared::VGarageError};

impl<R> VGarageFeature<R>
where
    R: VGarageRepository,
{
    pub(crate) fn get(&self, uid: &str) -> Result<Option<PlayerVGarage>, VGarageError> {
        self.service.get(uid)
    }
}
