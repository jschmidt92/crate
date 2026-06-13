use super::VGarageFeature;
use forge_lib::{
    models::{PlayerVGarage, VGarage},
    repositories::VGarageRepository,
    shared::VGarageError,
};

impl<R> VGarageFeature<R>
where
    R: VGarageRepository,
{
    pub(crate) fn init(&self, uid: &str, unlocks: &VGarage) -> Result<PlayerVGarage, VGarageError> {
        self.service.create_actor_garage(uid, unlocks)
    }

    pub(crate) fn disconnect(&self, uid: &str) -> Result<(), VGarageError> {
        self.service.disconnect(uid)
    }

    pub(crate) fn delete(&self, uid: &str) -> Result<(), VGarageError> {
        self.service.delete(uid)
    }
}
