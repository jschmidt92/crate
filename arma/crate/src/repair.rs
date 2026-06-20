use crate::{features::repair::RepairFeature, log, response};
use arma_rs::Group;
use forge_lib::{
    services::{BankService, RepairService},
    shared::{ServiceError, parse_non_negative_money},
};
use std::sync::LazyLock;

static REPAIR_FEATURE: LazyLock<RepairFeature<crate::persistence::CachedBankRepository>> =
    LazyLock::new(|| {
        RepairFeature::new(RepairService::new(BankService::new(
            crate::persistence::bank_repository(),
        )))
    });

pub fn group() -> Group {
    Group::new()
        .command("quote", repair_quote)
        .command("complete", repair_complete)
}

pub(crate) fn repair_quote(damage: String, full_repair_price: String) -> String {
    let Ok(damage) = parse_damage(&damage) else {
        return format!("Error: {}", ServiceError::InvalidDamage);
    };
    let Ok(full_repair_price) = parse_non_negative_money(&full_repair_price) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };

    match REPAIR_FEATURE.quote(damage, full_repair_price) {
        Ok(quote) => response::json(&quote, "repair quote"),
        Err(error) => {
            log::error(format_args!("failed to quote repair: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn repair_complete(
    uid: String,
    plate: String,
    damage: String,
    full_repair_price: String,
) -> String {
    let Ok(damage) = parse_damage(&damage) else {
        return format!("Error: {}", ServiceError::InvalidDamage);
    };
    let Ok(full_repair_price) = parse_non_negative_money(&full_repair_price) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };

    match REPAIR_FEATURE.complete(&uid, &plate, damage, full_repair_price) {
        Ok(receipt) => response::json(&receipt, "repair receipt"),
        Err(error) => {
            log::error(format_args!("failed to complete repair for {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn parse_damage(value: &str) -> Result<f64, ServiceError> {
    value
        .parse::<f64>()
        .map_err(|_| ServiceError::InvalidDamage)
}
