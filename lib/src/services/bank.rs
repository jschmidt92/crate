use crate::{
    models::{
        Actor, FuelTransaction, PlayerBankProfile, PlayerBankProfileView, TransactionReceipt,
    },
    shared::BankError,
};

pub fn create_actor_account(actor: &Actor) -> Result<PlayerBankProfileView, BankError> {
    if actor.uid.trim().is_empty() {
        return Err(BankError::InvalidActorUid);
    }

    let profile = PlayerBankProfile::new(actor.uid.clone());

    Ok(PlayerBankProfileView::from(&profile))
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
    use crate::models::ActorSnapshot;

    #[test]
    fn create_actor_account_uses_actor_uid_as_string() {
        let actor = Actor::from_snapshot(ActorSnapshot::new("steam:local-dev", "Tester"));

        let profile = create_actor_account(&actor).expect("account should be created");

        assert_eq!(profile.uid, "steam:local-dev");
        assert_eq!(profile.account.uid, "steam:local-dev");
    }
}
