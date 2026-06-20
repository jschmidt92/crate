use crate::{features::bank::BankFeature, log, response};
use arma_rs::Group;
use forge_lib::{models::Money, services::BankService, shared::BankError};
use std::sync::LazyLock;

static BANK_FEATURE: LazyLock<BankFeature<crate::persistence::CachedBankRepository>> =
    LazyLock::new(|| BankFeature::new(BankService::new(crate::persistence::bank_repository())));

pub fn group() -> Group {
    Group::new()
        .command("init", init_bank)
        .command("get", get_bank)
        .command("deposit", deposit_bank)
        .command("withdraw", withdraw_bank)
        .command("transfer", transfer_bank)
        .command("add_earnings", add_earnings)
        .command("submit_earnings", submit_earnings)
        .command("change_pin", change_pin)
}

pub(crate) fn init_bank(uid: String, starting_cash: String, starting_bank: String) -> String {
    match BANK_FEATURE.init_player_account(&uid, &starting_cash, &starting_bank) {
        Ok(profile) => response::json(&profile, "bank profile"),
        Err(error) => {
            log::error(format_args!("failed to init bank profile {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn get_bank(uid: String) -> String {
    match BANK_FEATURE.get_account(&uid) {
        Ok(Some(profile)) => response::json(&profile, "bank profile"),
        Ok(None) => "null".to_string(),
        Err(error) => {
            log::error(format_args!("failed to get bank profile {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn deposit_bank(uid: String, amount: String) -> String {
    let Ok(amount) = parse_amount(&amount) else {
        return format!("Error: {}", BankError::InvalidAmount);
    };

    match BANK_FEATURE.deposit_to_account(&uid, amount) {
        Ok(profile) => response::json(&profile, "bank profile"),
        Err(error) => {
            log::error(format_args!(
                "failed to deposit to bank profile {uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn withdraw_bank(uid: String, amount: String) -> String {
    let Ok(amount) = parse_amount(&amount) else {
        return format!("Error: {}", BankError::InvalidAmount);
    };

    match BANK_FEATURE.withdraw_from_account(&uid, amount) {
        Ok(profile) => response::json(&profile, "bank profile"),
        Err(error) => {
            log::error(format_args!(
                "failed to withdraw from bank profile {uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn transfer_bank(from_uid: String, to_uid: String, amount: String) -> String {
    let Ok(amount) = parse_amount(&amount) else {
        return format!("Error: {}", BankError::InvalidAmount);
    };

    match BANK_FEATURE.transfer_between_accounts(&from_uid, &to_uid, amount) {
        Ok((from, to)) => response::json(
            &serde_json::json!({ "from": from, "to": to }),
            "bank transfer",
        ),
        Err(error) => {
            log::error(format_args!(
                "failed to transfer bank funds from {from_uid} to {to_uid}: {error}"
            ));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn add_earnings(uid: String, amount: String) -> String {
    let Ok(amount) = parse_amount(&amount) else {
        return format!("Error: {}", BankError::InvalidAmount);
    };

    match BANK_FEATURE.add_pending_earnings(&uid, amount) {
        Ok(profile) => response::json(&profile, "bank profile"),
        Err(error) => {
            log::error(format_args!("failed to add earnings for {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn submit_earnings(uid: String) -> String {
    match BANK_FEATURE.submit_pending_earnings(&uid) {
        Ok(profile) => response::json(&profile, "bank profile"),
        Err(error) => {
            log::error(format_args!("failed to submit earnings for {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

pub(crate) fn change_pin(uid: String, current_pin: String, new_pin: String) -> String {
    match BANK_FEATURE.change_pin(&uid, &current_pin, &new_pin) {
        Ok(profile) => response::json(&profile, "bank profile"),
        Err(error) => {
            log::error(format_args!("failed to change ATM PIN for {uid}: {error}"));
            format!("Error: {error}")
        }
    }
}

fn parse_amount(amount: &str) -> Result<Money, BankError> {
    amount
        .parse::<Money>()
        .map_err(|_| BankError::InvalidAmount)
}
