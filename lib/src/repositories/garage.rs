use crate::{
    models::PlayerGarage,
    shared::{GarageError, StorageError},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub trait GarageRepository: Send + Sync {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerGarage>, GarageError>;
    fn save(&self, garage: PlayerGarage) -> Result<PlayerGarage, GarageError>;
    fn delete(&self, uid: &str) -> Result<(), GarageError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryGarageRepository {
    garages: Arc<RwLock<HashMap<String, PlayerGarage>>>,
}

impl InMemoryGarageRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl GarageRepository for InMemoryGarageRepository {
    fn find_by_uid(&self, uid: &str) -> Result<Option<PlayerGarage>, GarageError> {
        let garages = self.garages.read().map_storage_error()?;
        Ok(garages.get(uid).cloned())
    }

    fn save(&self, garage: PlayerGarage) -> Result<PlayerGarage, GarageError> {
        let mut garages = self.garages.write().map_storage_error()?;
        garage.garage.validate()?;
        garages.insert(garage.uid.clone(), garage.clone());
        Ok(garage)
    }

    fn delete(&self, uid: &str) -> Result<(), GarageError> {
        let mut garages = self.garages.write().map_storage_error()?;
        garages.remove(uid);
        Ok(())
    }
}
