use crate::shared::ServiceError;

pub fn validate_plate(plate: &str) -> Result<(), ServiceError> {
    if plate.trim().is_empty() {
        return Err(ServiceError::InvalidPlate);
    }

    Ok(())
}

pub fn validate_uid(uid: &str) -> Result<(), ServiceError> {
    if uid.trim().is_empty() {
        return Err(ServiceError::InvalidUid);
    }

    Ok(())
}
