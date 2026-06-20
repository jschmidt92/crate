// use arma_rs::{Context, Extension, arma};
use arma_rs::{Extension, arma};
use std::sync::LazyLock;
use tokio::runtime::{Builder, Runtime};
// use tokio::sync::RwLock as TokioRwLock;

mod actor;
mod bank;
mod command;
mod config;
mod events;
mod features;
mod garage;
mod locker;
mod log;
mod medical;
mod notification;
mod organization;
mod persistence;
mod rearm;
mod refuel;
mod repair;
mod response;
mod transport;
mod v_garage;
mod v_locker;

use actor::group as actor_group;
use bank::group as bank_group;
use garage::group as garage_group;
use locker::group as locker_group;
use medical::group as medical_group;
use notification::group as notification_group;
use organization::group as organization_group;
use rearm::group as rearm_group;
use refuel::group as refuel_group;
use repair::group as repair_group;
use transport::group as transport_group;
use v_garage::group as v_garage_group;
use v_locker::group as v_locker_group;

// static CONTEXT: LazyLock<TokioRwLock<Option<Context>>> = LazyLock::new(|| TokioRwLock::new(None));
pub(crate) static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build tokio runtime")
});

#[arma]
fn init() -> Extension {
    log::init();
    log::info(format_args!("initializing {}", get_version()));
    let config = config::load();
    persistence::init(config.database);
    events::init();

    Extension::build()
        .command("version", get_version)
        .command("status", get_status)
        .command("database_status", get_database_status)
        .command("config_path", get_config_path)
        .command("log_path", get_log_path)
        .group("actor", actor_group())
        .group("bank", bank_group())
        .group("refuel", refuel_group())
        .group("garage", garage_group())
        .group("locker", locker_group())
        .group("medical", medical_group())
        .group("notification", notification_group())
        .group("organization", organization_group())
        .group("rearm", rearm_group())
        .group("repair", repair_group())
        .group("transport", transport_group())
        .group("v_garage", v_garage_group())
        .group("v_locker", v_locker_group())
        .finish()
}

fn get_status() -> String {
    log::info(format_args!("status requested"));
    "Server is running".to_string()
}

fn get_database_status() -> String {
    persistence::status()
}

fn get_config_path() -> String {
    config::path().display().to_string()
}

fn get_version() -> String {
    format!("forge-crate v{}", env!("CARGO_PKG_VERSION"))
}

fn get_log_path() -> String {
    let path = log::path().display().to_string();
    log::info(format_args!("log_path requested: {path}"));
    path
}
