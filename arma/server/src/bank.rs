use crate::log;
use arma_rs::Group;
use forge_lib::{
    models::{Money, PlayerBankProfileView},
    services::BankService,
    shared::BankError,
};
use std::sync::LazyLock;

static BANK_SERVICE: LazyLock<BankService<crate::persistence::CachedBankRepository>> =
    LazyLock::new(|| BankService::new(crate::persistence::bank_repository()));

pub fn group() -> Group {
    Group::new()
        .command("init", init_bank)
        .command("disconnect", disconnect_bank)
}

pub(crate) fn init_bank(uid: String, starting_cash: String, starting_bank: String) -> String {
    match BANK_SERVICE.init_player_account(&uid, &starting_cash, &starting_bank) {
        Ok(profile) => match serde_json::to_string(&profile) {
            Ok(json) => json,
            Err(error) => {
                log::error(format_args!("failed to serialize bank profile: {error}"));
                format!("Error: failed to serialize bank profile: {error}")
            }
        },
        Err(error) => {
            log::error(format_args!("failed to init bank profile {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn disconnect_bank(uid: String) -> String {
    match BANK_SERVICE.disconnect_player_account(&uid) {
        Ok(()) => "OK".to_string(),
        Err(error) => {
            log::error(format_args!(
                "failed to disconnect bank profile {uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn deposit_payday(uid: &str, amount: Money) -> Result<PlayerBankProfileView, BankError> {
    BANK_SERVICE.deposit_to_account(uid, amount)
}
