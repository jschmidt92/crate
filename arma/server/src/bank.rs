use crate::log;
use arma_rs::Group;
use forge_lib::services::create_player_account;

pub fn group() -> Group {
    Group::new().command("init", init_bank)
}

fn init_bank(uid: String, starting_cash: String, starting_bank: String) -> String {
    match create_player_account(&uid, &starting_cash, &starting_bank) {
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
