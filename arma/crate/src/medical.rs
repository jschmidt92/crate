use crate::{features::medical::MedicalFeature, log, response};
use arma_rs::Group;
use forge_lib::{
    services::{BankService, MedicalService},
    shared::{ServiceError, parse_non_negative_money},
};
use std::sync::LazyLock;

static MEDICAL_FEATURE: LazyLock<MedicalFeature<crate::persistence::CachedBankRepository>> =
    LazyLock::new(|| {
        MedicalFeature::new(MedicalService::new(BankService::new(
            crate::persistence::bank_repository(),
        )))
    });

pub fn group() -> Group {
    Group::new()
        .command("respawn", medical_respawn)
        .command("heal", medical_heal)
}

pub(crate) fn medical_respawn(uid: String, respawn_price: String) -> String {
    let Ok(respawn_price) = parse_non_negative_money(&respawn_price) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };

    match MEDICAL_FEATURE.record_respawn(&uid, respawn_price) {
        Ok(receipt) => response::json(&receipt, "medical respawn receipt"),
        Err(error) => {
            log::error(format_args!("failed to record respawn for {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn medical_heal(uid: String, full_heal_price: String) -> String {
    let Ok(full_heal_price) = parse_non_negative_money(&full_heal_price) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };

    match MEDICAL_FEATURE.full_heal(&uid, full_heal_price) {
        Ok(receipt) => response::json(&receipt, "medical heal receipt"),
        Err(error) => {
            log::error(format_args!("failed to heal {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}
