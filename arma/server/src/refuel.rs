use crate::{features::refuel::FuelFeature, log};
use arma_rs::{Context, ContextState, Group};
use forge_lib::{
    models::FuelType,
    services::{BankService, FuelService},
    shared::ServiceError,
};
use std::{collections::HashMap, sync::RwLock};

static FUEL_FEATURE: std::sync::LazyLock<FuelFeature<crate::persistence::CachedBankRepository>> =
    std::sync::LazyLock::new(|| {
        FuelFeature::new(FuelService::new(BankService::new(
            crate::persistence::bank_repository(),
        )))
    });

pub fn group() -> Group {
    Group::new()
        .command("started", started)
        .command("tick", tick)
        .command("stopped", stopped)
        .command("price", price)
        .command("quote", quote)
        .command("complete", refuel_complete)
        .state(Fueling::default())
}

type FuelingState = RwLock<HashMap<(String, String), FuelingSession>>;

#[derive(Debug, Clone)]
struct FuelingSession {
    amount: f64,
    uid: String,
    plate: String,
    fuel_type: FuelType,
}

#[derive(Default)]
pub struct Fueling(FuelingState);

impl Fueling {
    const fn as_ref(&self) -> &FuelingState {
        &self.0
    }
}

fn started(
    ctx: Context,
    source: String,
    target: String,
    uid: String,
    plate: String,
    fuel_type: String,
) {
    if uid.trim().is_empty() {
        log::error(format_args!("Invalid uid: {}", uid));
        return;
    }
    let fueling = ctx
        .group()
        .get::<Fueling>()
        .expect("Unable to get Fueling state");
    let mut fueling = fueling
        .as_ref()
        .write()
        .expect("Unable to acquire write lock on Fueling state");
    let fuel_type = FuelType::try_from(fuel_type.as_str()).unwrap_or_else(|()| {
        log::error(format_args!(
            "Invalid fuel_type: {}, defaulting to regular",
            fuel_type
        ));
        FuelType::Regular
    });
    log::info(format_args!(
        "Started fueling from {} to {} for uid {}",
        source, target, uid
    ));
    fueling.insert(
        (source, target),
        FuelingSession {
            amount: 0.0,
            uid,
            plate,
            fuel_type,
        },
    );
}

fn tick(ctx: Context, source: String, target: String, amount: f64) {
    let fueling = ctx
        .group()
        .get::<Fueling>()
        .expect("Unable to get Fueling state");
    let mut fueling = fueling
        .as_ref()
        .write()
        .expect("Unable to acquire write lock on Fueling state");
    let entry = fueling
        .entry((source.clone(), target))
        .or_insert(FuelingSession {
            amount: 0.0,
            uid: String::new(),
            plate: String::new(),
            fuel_type: FuelType::Regular,
        });
    log::info(format_args!("Tick fueling from {}: +{}", source, amount));
    entry.amount += amount;
}

fn stopped(ctx: Context, source: String, target: String) {
    log::info(format_args!(
        "Stopped fueling from {} to {}",
        source, target
    ));
    let session = {
        let fueling = ctx
            .group()
            .get::<Fueling>()
            .expect("Unable to get Fueling state");
        let mut fueling = fueling
            .as_ref()
            .write()
            .expect("Unable to acquire write lock on Fueling state");
        let Some(session) = fueling.remove(&(source.clone(), target.clone())) else {
            log::error(format_args!(
                "No matching entry found for {} to {}",
                source, target
            ));
            return;
        };
        session
    };

    match FUEL_FEATURE.complete(
        &session.uid,
        &session.plate,
        session.amount,
        session.fuel_type,
    ) {
        Ok(receipt) => {
            log::info(format_args!(
                "Completed refuel for uid {}: {} ({})",
                receipt.uid,
                receipt.amount.as_str(),
                receipt.description
            ));
        }
        Err(error) => {
            log::error(format_args!("Failed to complete refuel: {error}"));
        }
    }
}

fn price(fuel_type: String) -> f64 {
    let price =
        FUEL_FEATURE.price(FuelType::try_from(fuel_type.as_str()).unwrap_or(FuelType::Regular));
    log::info(format_args!(
        "Fuel price requested for {}: {:.2}",
        fuel_type, price
    ));
    price
}

pub(crate) fn quote(liters: String, fuel_type: String) -> String {
    let Ok(liters) = parse_f64(&liters) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };
    let Ok(fuel_type) = parse_fuel_type(&fuel_type) else {
        return format!("Error: {}", ServiceError::InvalidFuelType);
    };

    match FUEL_FEATURE.quote(liters, fuel_type) {
        Ok(quote) => serialize_refuel(&quote, "refuel quote"),
        Err(error) => {
            log::error(format_args!("failed to quote refuel: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn refuel_complete(
    uid: String,
    plate: String,
    liters: String,
    fuel_type: String,
) -> String {
    let Ok(liters) = parse_f64(&liters) else {
        return format!("Error: {}", ServiceError::InvalidAmount);
    };
    let Ok(fuel_type) = parse_fuel_type(&fuel_type) else {
        return format!("Error: {}", ServiceError::InvalidFuelType);
    };

    match FUEL_FEATURE.complete(&uid, &plate, liters, fuel_type) {
        Ok(receipt) => serialize_refuel(&receipt, "refuel receipt"),
        Err(error) => {
            log::error(format_args!("failed to complete refuel for {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn parse_f64(value: &str) -> Result<f64, ServiceError> {
    value
        .parse::<f64>()
        .map_err(|_| ServiceError::InvalidAmount)
}

fn parse_fuel_type(value: &str) -> Result<FuelType, ServiceError> {
    FuelType::try_from(value).map_err(|()| ServiceError::InvalidFuelType)
}

fn serialize_refuel<T>(value: &T, label: &str) -> String
where
    T: serde::Serialize,
{
    serde_json::to_string(value).unwrap_or_else(|error| {
        log::error(format_args!("failed to serialize {label}: {error}"));
        format!("Error: failed to serialize {label}: {error}")
    })
}
