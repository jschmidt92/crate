use crate::{models::Money, shared::ServiceError};

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

pub fn parse_non_negative_money(value: &str) -> Result<Money, ServiceError> {
    let money = value
        .parse::<Money>()
        .map_err(|_| ServiceError::InvalidAmount)?;
    if money.cents() < 0 {
        return Err(ServiceError::InvalidAmount);
    }
    Ok(money)
}
