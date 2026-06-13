use crate::{features::rearm::RearmFeature, log, response};
use arma_rs::Group;
use forge_lib::{
    services::{BankService, RearmService},
    shared::{ServiceError, parse_non_negative_money},
};
use std::sync::LazyLock;

static REARM_FEATURE: LazyLock<RearmFeature<crate::persistence::CachedBankRepository>> =
    LazyLock::new(|| {
        RearmFeature::new(RearmService::new(BankService::new(
            crate::persistence::bank_repository(),
        )))
    });

pub fn group() -> Group {
    Group::new()
        .command("quote", rearm_quote)
        .command("complete", rearm_complete)
}

pub(crate) fn rearm_quote(units: String, unit_price: String) -> String {
    let Ok(units) = parse_units(&units) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };
    let Ok(unit_price) = parse_non_negative_money(&unit_price) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };

    match REARM_FEATURE.quote(units, unit_price) {
        Ok(quote) => response::json(&quote, "rearm quote"),
        Err(error) => {
            log::error(format_args!("failed to quote rearm: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn rearm_complete(
    uid: String,
    plate: String,
    units: String,
    unit_price: String,
) -> String {
    let Ok(units) = parse_units(&units) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };
    let Ok(unit_price) = parse_non_negative_money(&unit_price) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };

    match REARM_FEATURE.complete(&uid, &plate, units, unit_price) {
        Ok(receipt) => response::json(&receipt, "rearm receipt"),
        Err(error) => {
            log::error(format_args!("failed to complete rearm for {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn parse_units(value: &str) -> Result<u32, ServiceError> {
    value
        .parse::<u32>()
        .map_err(|_| ServiceError::InvalidAmount)
}
