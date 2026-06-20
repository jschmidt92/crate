use crate::{
    models::{
        Actor, ActorStartingConfig, FuelTransaction, Money, PlayerBankProfile,
        PlayerBankProfileView, TransactionReceipt,
    },
    repositories::BankRepository,
    shared::BankError,
};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct BankService<R> {
    repository: R,
}

impl<R> BankService<R>
where
    R: BankRepository,
{
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn init_player_account(
        &self,
        uid: &str,
        starting_cash: &str,
        starting_bank: &str,
    ) -> Result<PlayerBankProfileView, BankError> {
        validate_uid(uid)?;

        if let Some(profile) = self.repository.find_by_uid(uid)? {
            return Ok(PlayerBankProfileView::from(&profile));
        }

        let cash = parse_starting_money(starting_cash)?;
        let bank = parse_starting_money(starting_bank)?;
        let profile = PlayerBankProfile::with_starting_balances(uid.to_string(), cash, bank);

        Ok(PlayerBankProfileView::from(&self.repository.save(profile)?))
    }

    pub fn deposit_to_account(
        &self,
        uid: &str,
        amount: Money,
    ) -> Result<PlayerBankProfileView, BankError> {
        validate_uid(uid)?;
        if !amount.is_positive() {
            return Err(BankError::InvalidAmount);
        }

        let mut profile = self
            .repository
            .find_by_uid(uid)?
            .unwrap_or_else(|| PlayerBankProfile::new(uid));
        profile.account.deposit(amount);
        profile.record_transaction(amount, "Account credit");

        Ok(PlayerBankProfileView::from(&self.repository.save(profile)?))
    }

    pub fn get_account(&self, uid: &str) -> Result<Option<PlayerBankProfileView>, BankError> {
        validate_uid(uid)?;

        Ok(self
            .repository
            .find_by_uid(uid)?
            .as_ref()
            .map(PlayerBankProfileView::from))
    }

    pub fn withdraw_from_account(
        &self,
        uid: &str,
        amount: Money,
    ) -> Result<PlayerBankProfileView, BankError> {
        validate_uid(uid)?;
        if !amount.is_positive() {
            return Err(BankError::InvalidAmount);
        }

        let mut profile = self
            .repository
            .find_by_uid(uid)?
            .ok_or(BankError::InsufficientFunds)?;

        if !profile.account.withdraw(amount) {
            return Err(BankError::InsufficientFunds);
        }
        profile.record_transaction(Money::from_cents(-amount.cents()), "Account debit");

        Ok(PlayerBankProfileView::from(&self.repository.save(profile)?))
    }

    pub fn deposit_cash_to_account(
        &self,
        uid: &str,
        amount: Money,
    ) -> Result<PlayerBankProfileView, BankError> {
        validate_uid(uid)?;
        if !amount.is_positive() {
            return Err(BankError::InvalidAmount);
        }

        let mut profile = self
            .repository
            .find_by_uid(uid)?
            .ok_or(BankError::InsufficientFunds)?;
        if profile.cash < amount {
            return Err(BankError::InsufficientCash);
        }

        profile.cash = profile.cash - amount;
        profile.account.deposit(amount);
        profile.record_transaction(amount, "Cash deposit");

        Ok(PlayerBankProfileView::from(&self.repository.save(profile)?))
    }

    pub fn withdraw_cash_from_account(
        &self,
        uid: &str,
        amount: Money,
    ) -> Result<PlayerBankProfileView, BankError> {
        validate_uid(uid)?;
        if !amount.is_positive() {
            return Err(BankError::InvalidAmount);
        }

        let mut profile = self
            .repository
            .find_by_uid(uid)?
            .ok_or(BankError::InsufficientFunds)?;
        if !profile.account.withdraw(amount) {
            return Err(BankError::InsufficientFunds);
        }

        profile.cash = profile.cash + amount;
        profile.record_transaction(Money::from_cents(-amount.cents()), "Cash withdrawal");

        Ok(PlayerBankProfileView::from(&self.repository.save(profile)?))
    }

    pub fn add_pending_earnings(
        &self,
        uid: &str,
        amount: Money,
    ) -> Result<PlayerBankProfileView, BankError> {
        validate_uid(uid)?;
        if !amount.is_positive() {
            return Err(BankError::InvalidAmount);
        }

        let mut profile = self
            .repository
            .find_by_uid(uid)?
            .unwrap_or_else(|| PlayerBankProfile::new(uid));
        profile.pending_earnings = profile.pending_earnings + amount;

        Ok(PlayerBankProfileView::from(&self.repository.save(profile)?))
    }

    pub fn submit_pending_earnings(&self, uid: &str) -> Result<PlayerBankProfileView, BankError> {
        validate_uid(uid)?;
        let mut profile = self
            .repository
            .find_by_uid(uid)?
            .ok_or(BankError::InvalidAmount)?;
        let amount = profile.pending_earnings;
        if !amount.is_positive() {
            return Err(BankError::InvalidAmount);
        }

        profile.pending_earnings = Money::ZERO;
        profile.account.deposit(amount);
        profile.record_transaction(amount, "Submitted earnings");

        Ok(PlayerBankProfileView::from(&self.repository.save(profile)?))
    }

    pub fn change_pin(
        &self,
        uid: &str,
        current_pin: &str,
        new_pin: &str,
    ) -> Result<PlayerBankProfileView, BankError> {
        validate_uid(uid)?;
        validate_pin(new_pin)?;

        let mut profile = self
            .repository
            .find_by_uid(uid)?
            .ok_or(BankError::InvalidActorUid)?;
        if profile.pin_is_set() && !profile.verify_pin(current_pin) {
            return Err(BankError::IncorrectPin);
        }

        profile.set_pin(new_pin);
        Ok(PlayerBankProfileView::from(&self.repository.save(profile)?))
    }

    pub fn transfer_between_accounts(
        &self,
        from_uid: &str,
        to_uid: &str,
        amount: Money,
    ) -> Result<(PlayerBankProfileView, PlayerBankProfileView), BankError> {
        validate_uid(from_uid)?;
        validate_uid(to_uid)?;
        if from_uid == to_uid {
            return Err(BankError::InvalidActorUid);
        }
        if !amount.is_positive() {
            return Err(BankError::InvalidAmount);
        }

        let mut from_profile = self
            .repository
            .find_by_uid(from_uid)?
            .ok_or(BankError::InsufficientFunds)?;

        if !from_profile.account.withdraw(amount) {
            return Err(BankError::InsufficientFunds);
        }
        from_profile.record_transaction(
            Money::from_cents(-amount.cents()),
            format!("Transfer to {to_uid}"),
        );

        let mut to_profile = self
            .repository
            .find_by_uid(to_uid)?
            .unwrap_or_else(|| PlayerBankProfile::new(to_uid));
        to_profile.account.deposit(amount);
        to_profile.record_transaction(amount, format!("Transfer from {from_uid}"));

        let Ok([from_profile, to_profile]) = <[PlayerBankProfile; 2]>::try_from(
            self.repository.save_many(vec![from_profile, to_profile])?,
        ) else {
            return Err(BankError::Repository(
                "bank transfer did not persist both profiles".to_string(),
            ));
        };

        Ok((
            PlayerBankProfileView::from(&from_profile),
            PlayerBankProfileView::from(&to_profile),
        ))
    }

    pub fn prepare_deposits(
        &self,
        deposits: &[(String, Money)],
    ) -> Result<Vec<PlayerBankProfile>, BankError> {
        let mut totals = BTreeMap::new();

        for (uid, amount) in deposits {
            validate_uid(uid)?;
            if !amount.is_positive() {
                return Err(BankError::InvalidAmount);
            }

            let total = totals.entry(uid.clone()).or_insert(Money::ZERO);
            *total = *total + *amount;
        }

        let mut profiles = Vec::with_capacity(totals.len());
        for (uid, amount) in totals {
            let mut profile = self
                .repository
                .find_by_uid(&uid)?
                .unwrap_or_else(|| PlayerBankProfile::new(&uid));
            profile.account.deposit(amount);
            profile.record_transaction(amount, "Organization payday");
            profiles.push(profile);
        }

        Ok(profiles)
    }

    pub fn disconnect_player_account(&self, uid: &str) -> Result<(), BankError> {
        validate_uid(uid)?;
        Ok(())
    }
}

pub fn create_actor_account(
    actor: &Actor,
    starting: &ActorStartingConfig,
) -> Result<PlayerBankProfileView, BankError> {
    if actor.uid.trim().is_empty() {
        return Err(BankError::InvalidActorUid);
    }

    let cash = parse_starting_money(&starting.cash)?;
    let bank = parse_starting_money(&starting.bank)?;
    let profile = PlayerBankProfile::with_starting_balances(actor.uid.clone(), cash, bank);

    Ok(PlayerBankProfileView::from(&profile))
}

pub fn create_player_account(
    uid: &str,
    starting_cash: &str,
    starting_bank: &str,
) -> Result<PlayerBankProfileView, BankError> {
    validate_uid(uid)?;

    let cash = parse_starting_money(starting_cash)?;
    let bank = parse_starting_money(starting_bank)?;
    let profile = PlayerBankProfile::with_starting_balances(uid.to_string(), cash, bank);

    Ok(PlayerBankProfileView::from(&profile))
}

pub fn disconnect_player_account(uid: &str) -> Result<(), BankError> {
    validate_uid(uid)
}

fn validate_uid(uid: &str) -> Result<(), BankError> {
    if uid.trim().is_empty() {
        return Err(BankError::InvalidActorUid);
    }

    Ok(())
}

fn validate_pin(pin: &str) -> Result<(), BankError> {
    if (4..=6).contains(&pin.len()) && pin.bytes().all(|byte| byte.is_ascii_digit()) {
        Ok(())
    } else {
        Err(BankError::InvalidPin)
    }
}

fn parse_starting_money(amount: &str) -> Result<Money, BankError> {
    let money = amount
        .parse::<Money>()
        .map_err(|_| BankError::InvalidAmount)?;

    if money.cents() < 0 {
        return Err(BankError::InvalidAmount);
    }

    Ok(money)
}

pub async fn process_fuel_transaction(
    transaction: FuelTransaction,
) -> Result<TransactionReceipt, BankError> {
    let total = transaction.total();
    if !total.is_finite() || total <= 0.0 {
        return Err(BankError::InvalidAmount);
    }

    // This is the persistence boundary for the future bank repository.
    Ok(TransactionReceipt {
        uid: transaction.uid,
        amount: total,
        description: format!(
            "{:.2} liters of {} refuel for {}",
            transaction.liters, transaction.fuel_type, transaction.plate
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{ActorSnapshot, ActorStartingConfig},
        repositories::InMemoryBankRepository,
    };

    #[test]
    fn create_actor_account_uses_actor_uid_as_string() {
        let actor = Actor::from_snapshot(ActorSnapshot::new("steam:local-dev", "Tester"));

        let profile = create_actor_account(&actor, &ActorStartingConfig::default())
            .expect("account should be created");

        assert_eq!(profile.uid, "steam:local-dev");
        assert_eq!(profile.account.uid, "steam:local-dev");
    }

    #[test]
    fn create_actor_account_uses_configured_starting_money() {
        let actor = Actor::from_snapshot(ActorSnapshot::new("steam:local-dev", "Tester"));
        let starting = ActorStartingConfig {
            cash: "125.50".to_string(),
            bank: "250.25".to_string(),
            ..ActorStartingConfig::default()
        };

        let profile = create_actor_account(&actor, &starting).expect("account should be created");

        assert_eq!(profile.cash.as_str(), "125.50");
        assert_eq!(profile.account.balance.as_str(), "250.25");
    }

    #[test]
    fn create_actor_account_rejects_negative_starting_money() {
        let actor = Actor::from_snapshot(ActorSnapshot::new("steam:local-dev", "Tester"));
        let starting = ActorStartingConfig {
            cash: "-1.00".to_string(),
            ..ActorStartingConfig::default()
        };

        assert!(matches!(
            create_actor_account(&actor, &starting),
            Err(BankError::InvalidAmount)
        ));
    }

    #[test]
    fn create_player_account_uses_configured_starting_money() {
        let profile = create_player_account("steam:local-dev", "25.00", "100.00")
            .expect("account should be created");

        assert_eq!(profile.uid, "steam:local-dev");
        assert_eq!(profile.cash.as_str(), "25.00");
        assert_eq!(profile.account.balance.as_str(), "100.00");
    }

    #[test]
    fn disconnect_player_account_rejects_empty_uid() {
        assert!(matches!(
            disconnect_player_account(""),
            Err(BankError::InvalidActorUid)
        ));
    }

    #[test]
    fn bank_service_deposits_to_existing_account() {
        let service = BankService::new(InMemoryBankRepository::new());
        service
            .init_player_account("steam:local-dev", "0.00", "100.00")
            .expect("account should be created");

        let profile = service
            .deposit_to_account("steam:local-dev", Money::from_cents(2500))
            .expect("deposit should succeed");

        assert_eq!(profile.account.balance.as_str(), "125.00");
    }

    #[test]
    fn bank_service_deposit_creates_missing_account() {
        let service = BankService::new(InMemoryBankRepository::new());

        let profile = service
            .deposit_to_account("steam:offline", Money::from_cents(2500))
            .expect("deposit should create account");

        assert_eq!(profile.cash.as_str(), "0.00");
        assert_eq!(profile.account.balance.as_str(), "25.00");
    }

    #[test]
    fn bank_service_withdraws_from_existing_account() {
        let service = BankService::new(InMemoryBankRepository::new());
        service
            .init_player_account("steam:local-dev", "0.00", "100.00")
            .expect("account should be created");

        let profile = service
            .withdraw_from_account("steam:local-dev", Money::from_cents(2500))
            .expect("withdraw should succeed");

        assert_eq!(profile.account.balance.as_str(), "75.00");
    }

    #[test]
    fn bank_service_rejects_overdraft() {
        let service = BankService::new(InMemoryBankRepository::new());
        service
            .init_player_account("steam:local-dev", "0.00", "10.00")
            .expect("account should be created");

        assert!(matches!(
            service.withdraw_from_account("steam:local-dev", Money::from_cents(2500)),
            Err(BankError::InsufficientFunds)
        ));
    }

    #[test]
    fn bank_service_transfers_between_accounts() {
        let service = BankService::new(InMemoryBankRepository::new());
        service
            .init_player_account("steam:from", "0.00", "100.00")
            .expect("from account should be created");

        let (from, to) = service
            .transfer_between_accounts("steam:from", "steam:to", Money::from_cents(2500))
            .expect("transfer should succeed");

        assert_eq!(from.account.balance.as_str(), "75.00");
        assert_eq!(to.account.balance.as_str(), "25.00");
    }

    #[test]
    fn cash_deposit_moves_cash_into_account_and_records_ledger() {
        let service = BankService::new(InMemoryBankRepository::new());
        service
            .init_player_account("steam:local-dev", "50.00", "100.00")
            .expect("account should be created");

        let profile = service
            .deposit_cash_to_account("steam:local-dev", Money::from_cents(2500))
            .expect("cash deposit should succeed");

        assert_eq!(profile.cash.as_str(), "25.00");
        assert_eq!(profile.account.balance.as_str(), "125.00");
        assert_eq!(profile.transactions[0].description, "Cash deposit");
        assert_eq!(profile.transactions[0].amount.as_str(), "25.00");
    }

    #[test]
    fn cash_withdrawal_moves_account_funds_to_cash() {
        let service = BankService::new(InMemoryBankRepository::new());
        service
            .init_player_account("steam:local-dev", "50.00", "100.00")
            .expect("account should be created");

        let profile = service
            .withdraw_cash_from_account("steam:local-dev", Money::from_cents(2500))
            .expect("cash withdrawal should succeed");

        assert_eq!(profile.cash.as_str(), "75.00");
        assert_eq!(profile.account.balance.as_str(), "75.00");
        assert_eq!(profile.transactions[0].amount.as_str(), "-25.00");
    }

    #[test]
    fn pending_earnings_submit_to_account() {
        let service = BankService::new(InMemoryBankRepository::new());
        service
            .init_player_account("steam:local-dev", "0.00", "100.00")
            .expect("account should be created");
        service
            .add_pending_earnings("steam:local-dev", Money::from_cents(3400))
            .expect("earnings should be added");

        let profile = service
            .submit_pending_earnings("steam:local-dev")
            .expect("earnings should submit");

        assert_eq!(profile.pending_earnings.as_str(), "0.00");
        assert_eq!(profile.account.balance.as_str(), "134.00");
        assert_eq!(profile.transactions[0].description, "Submitted earnings");
    }

    #[test]
    fn pin_requires_current_pin_after_initial_setup() {
        let service = BankService::new(InMemoryBankRepository::new());
        service
            .init_player_account("steam:local-dev", "0.00", "100.00")
            .expect("account should be created");

        let profile = service
            .change_pin("steam:local-dev", "", "1234")
            .expect("initial PIN should be set");
        assert!(profile.pin_set);

        assert!(matches!(
            service.change_pin("steam:local-dev", "9999", "5678"),
            Err(BankError::IncorrectPin)
        ));
        service
            .change_pin("steam:local-dev", "1234", "5678")
            .expect("correct current PIN should permit change");
    }

    #[test]
    fn ledger_keeps_only_ten_most_recent_transactions() {
        let service = BankService::new(InMemoryBankRepository::new());
        for _ in 0..12 {
            service
                .deposit_to_account("steam:local-dev", Money::from_cents(100))
                .expect("credit should succeed");
        }

        let profile = service
            .get_account("steam:local-dev")
            .expect("read should succeed")
            .expect("profile should exist");
        assert_eq!(profile.transactions.len(), 10);
    }

    #[test]
    fn prepare_deposits_combines_duplicate_recipients() {
        let service = BankService::new(InMemoryBankRepository::new());
        service
            .init_player_account("steam:local-dev", "0.00", "10.00")
            .expect("account should be created");

        let profiles = service
            .prepare_deposits(&[
                ("steam:local-dev".to_string(), Money::from_cents(2500)),
                ("steam:local-dev".to_string(), Money::from_cents(1250)),
            ])
            .expect("deposits should prepare");

        assert_eq!(profiles.len(), 1);
        assert_eq!(profiles[0].account.balance, Money::from_cents(4750));
    }
}
