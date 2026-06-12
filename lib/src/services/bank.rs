use crate::{
    models::{
        Actor, ActorStartingConfig, FuelTransaction, Money, PlayerBankProfile,
        PlayerBankProfileView, TransactionReceipt,
    },
    shared::BankError,
};

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
            "{:.2} liters of {} fuel for {}",
            transaction.liters, transaction.fuel_type, transaction.plate
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ActorSnapshot, ActorStartingConfig};

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
}
