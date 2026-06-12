use crate::{RUNTIME, log};
use arma_rs::{Context, ContextState, Group};
use forge_lib::{
    models::{FuelTransaction, FuelType},
    services::bank,
};
use std::{collections::HashMap, sync::RwLock};

pub fn group() -> Group {
    Group::new()
        .command("started", started)
        .command("tick", tick)
        .command("stopped", stopped)
        .command("price", price)
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

    let price_per_liter = session.fuel_type.price_per_liter();
    let transaction = FuelTransaction {
        uid: session.uid,
        plate: session.plate,
        liters: session.amount,
        fuel_type: session.fuel_type,
        price_per_liter,
    };
    log::info(format_args!(
        "Queued fuel bank transaction for uid {} vehicle {}: {:.2} liters of {}",
        transaction.uid, transaction.plate, transaction.liters, transaction.fuel_type
    ));
    let _ = RUNTIME.spawn(async move {
        match bank::process_fuel_transaction(transaction).await {
            Ok(receipt) => {
                log::info(format_args!(
                    "Completed bank transaction for uid {}: {:.2} ({})",
                    receipt.uid, receipt.amount, receipt.description
                ));
            }
            Err(error) => {
                log::error(format_args!(
                    "Failed to process fuel bank transaction: {error}"
                ));
            }
        }
    });
}

fn price(fuel_type: String) -> f64 {
    let price = FuelType::try_from(fuel_type.as_str())
        .unwrap_or(FuelType::Regular)
        .price_per_liter();
    log::info(format_args!(
        "Fuel price requested for {}: {:.2}",
        fuel_type, price
    ));
    price
}
