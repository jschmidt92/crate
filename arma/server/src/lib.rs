// use arma_rs::{Context, Extension, arma};
use arma_rs::{Extension, arma};
use std::sync::LazyLock;
use tokio::runtime::{Builder, Runtime};
// use tokio::sync::RwLock as TokioRwLock;

mod actor;
mod fuel;
mod log;

use actor::group as actor_group;
use fuel::group as fuel_group;

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

    Extension::build()
        .command("version", get_version)
        .command("status", get_status)
        .command("log_path", get_log_path)
        .group("actor", actor_group())
        .group("fuel", fuel_group())
        .finish()
}

fn get_status() -> String {
    log::info(format_args!("status requested"));
    "Server is running".to_string()
}

fn get_version() -> String {
    format!("forge-server v{}", env!("CARGO_PKG_VERSION"))
}

fn get_log_path() -> String {
    let path = log::path().display().to_string();
    log::info(format_args!("log_path requested: {path}"));
    path
}
