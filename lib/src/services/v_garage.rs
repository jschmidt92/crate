use crate::{
    models::{PlayerVGarage, VGarage},
    repositories::VGarageRepository,
    shared::VGarageError,
};

#[derive(Clone)]
pub struct VGarageService<R> {
    repository: R,
}

impl<R> VGarageService<R>
where
    R: VGarageRepository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create_actor_garage(
        &self,
        uid: &str,
        starting_unlocks: &VGarage,
    ) -> Result<PlayerVGarage, VGarageError> {
        validate_uid(uid)?;

        if let Some(mut garage) = self.repository.find_by_uid(uid)? {
            garage.unlocks.merge(starting_unlocks);
            return self.repository.save(garage);
        }

        self.repository
            .save(PlayerVGarage::new(uid, starting_unlocks.clone()))
    }

    pub fn get(&self, uid: &str) -> Result<Option<PlayerVGarage>, VGarageError> {
        validate_uid(uid)?;
        self.repository.find_by_uid(uid)
    }

    pub fn save(&self, garage: PlayerVGarage) -> Result<PlayerVGarage, VGarageError> {
        validate_uid(&garage.uid)?;
        self.repository.save(garage)
    }

    pub fn delete(&self, uid: &str) -> Result<(), VGarageError> {
        validate_uid(uid)?;
        self.repository.delete(uid)
    }
}

fn validate_uid(uid: &str) -> Result<(), VGarageError> {
    if uid.trim().is_empty() {
        return Err(VGarageError::InvalidUid);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryVGarageRepository;

    #[test]
    fn create_actor_garage_provisions_starting_unlocks() {
        let service = VGarageService::new(InMemoryVGarageRepository::new());
        let starting = VGarage {
            cars: vec!["B_Quadbike_01_F".to_string()],
            ..VGarage::default()
        };

        let garage = service
            .create_actor_garage("steam:local-dev", &starting)
            .expect("garage should be created");

        assert_eq!(garage.uid, "steam:local-dev");
        assert_eq!(garage.unlocks.cars, ["B_Quadbike_01_F"]);
    }

    #[test]
    fn create_actor_garage_merges_existing_garage() {
        let service = VGarageService::new(InMemoryVGarageRepository::new());
        let first = VGarage {
            cars: vec!["B_Quadbike_01_F".to_string()],
            ..VGarage::default()
        };
        let second = VGarage {
            cars: vec!["B_Quadbike_01_F".to_string(), "C_Offroad_01_F".to_string()],
            ..VGarage::default()
        };

        service
            .create_actor_garage("steam:local-dev", &first)
            .expect("garage should be created");
        let garage = service
            .create_actor_garage("steam:local-dev", &second)
            .expect("garage should be updated");

        assert_eq!(garage.unlocks.cars, ["B_Quadbike_01_F", "C_Offroad_01_F"]);
    }
}
