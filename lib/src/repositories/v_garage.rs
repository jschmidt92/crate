use crate::{
    models::PlayerVGarage,
    shared::{StorageError, VGarageError},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub trait VGarageRepository: Send + Sync {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerVGarage>, VGarageError>;
    fn save(&self, garage: PlayerVGarage) -> Result<PlayerVGarage, VGarageError>;
    fn delete(&self, uid: &str) -> Result<(), VGarageError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryVGarageRepository {
    garages: Arc<RwLock<HashMap<String, PlayerVGarage>>>,
}

impl InMemoryVGarageRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl VGarageRepository for InMemoryVGarageRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerVGarage>, VGarageError> {
        let garages = self.garages.read().map_storage_error()?;

        Ok(garages.get(uid).cloned())
    }

    fn save(&self, garage: PlayerVGarage) -> Result<PlayerVGarage, VGarageError> {
        let mut garages = self.garages.write().map_storage_error()?;

        garages.insert(garage.uid.clone(), garage.clone());
        Ok(garage)
    }

    fn delete(&self, uid: &str) -> Result<(), VGarageError> {
        let mut garages = self.garages.write().map_storage_error()?;

        garages.remove(uid);
        Ok(())
    }
}
