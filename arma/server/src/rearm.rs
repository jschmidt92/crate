use crate::{features::rearm::RearmFeature, log};
use arma_rs::Group;
use forge_lib::{
    services::{BankService, RearmService},
    shared::ServiceError,
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

pub(crate) fn rearm_quote(units: String) -> String {
    let Ok(units) = parse_units(&units) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };

    match REARM_FEATURE.quote(units) {
        Ok(quote) => serialize_rearm(&quote, "rearm quote"),
        Err(error) => {
            log::error(format_args!("failed to quote rearm: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn rearm_complete(uid: String, plate: String, units: String) -> String {
    let Ok(units) = parse_units(&units) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };

    match REARM_FEATURE.complete(&uid, &plate, units) {
        Ok(receipt) => serialize_rearm(&receipt, "rearm receipt"),
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

fn serialize_rearm<T>(value: &T, label: &str) -> String
where
    T: serde::Serialize,
{
    serde_json::to_string(value).unwrap_or_else(|error| {
        log::error(format_args!("failed to serialize {label}: {error}"));
        format!("Error: failed to serialize {label}: {error}")
    })
}
