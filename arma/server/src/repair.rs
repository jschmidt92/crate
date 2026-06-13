use crate::{features::repair::RepairFeature, log};
use arma_rs::Group;
use forge_lib::{
    services::{BankService, RepairService},
    shared::ServiceError,
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

pub(crate) fn repair_quote(damage: String) -> String {
    let Ok(damage) = parse_damage(&damage) else {
        return format!("Error: {}", ServiceError::InvalidDamage);
    };

    match REPAIR_FEATURE.quote(damage) {
        Ok(quote) => serialize_repair(&quote, "repair quote"),
        Err(error) => {
            log::error(format_args!("failed to quote repair: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn repair_complete(uid: String, plate: String, damage: String) -> String {
    let Ok(damage) = parse_damage(&damage) else {
        return format!("Error: {}", ServiceError::InvalidDamage);
    };

    match REPAIR_FEATURE.complete(&uid, &plate, damage) {
        Ok(receipt) => serialize_repair(&receipt, "repair receipt"),
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

fn serialize_repair<T>(value: &T, label: &str) -> String
where
    T: serde::Serialize,
{
    serde_json::to_string(value).unwrap_or_else(|error| {
        log::error(format_args!("failed to serialize {label}: {error}"));
        format!("Error: failed to serialize {label}: {error}")
    })
}
