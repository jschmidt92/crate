use crate::{models::PlayerGarage, repositories::GarageRepository, shared::GarageError};

#[derive(Clone)]
pub struct GarageService<R> {
    repository: R,
}

impl<R> GarageService<R>
where
    R: GarageRepository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create_actor_garage(&self, uid: &str) -> Result<PlayerGarage, GarageError> {
        validate_uid(uid)?;

        if let Some(garage) = self.repository.find_by_uid(uid)? {
            return Ok(garage);
        }

        self.repository.save(PlayerGarage::new(uid))
    }

    pub fn get(&self, uid: &str) -> Result<Option<PlayerGarage>, GarageError> {
        validate_uid(uid)?;
        self.repository.find_by_uid(uid)
    }

    pub fn save(&self, garage: PlayerGarage) -> Result<PlayerGarage, GarageError> {
        validate_uid(&garage.uid)?;
        self.repository.save(garage)
    }

    pub fn delete(&self, uid: &str) -> Result<(), GarageError> {
        validate_uid(uid)?;
        self.repository.delete(uid)
    }

    pub fn disconnect(&self, uid: &str) -> Result<(), GarageError> {
        validate_uid(uid)?;
        Ok(())
    }
}

fn validate_uid(uid: &str) -> Result<(), GarageError> {
    if uid.trim().is_empty() {
        return Err(GarageError::InvalidUid);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryGarageRepository;

    #[test]
    fn create_actor_garage_creates_empty_garage() {
        let service = GarageService::new(InMemoryGarageRepository::new());

        let garage = service
            .create_actor_garage("steam:local-dev")
            .expect("garage should be created");

        assert_eq!(garage.uid, "steam:local-dev");
        assert!(garage.garage.vehicles.is_empty());
    }
}
