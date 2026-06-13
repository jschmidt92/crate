use crate::{
    models::{Money, ServiceKind, ServiceReceipt},
    repositories::BankRepository,
    services::BankService,
    shared::ServiceError,
};

const FULL_HEAL_PRICE: f64 = 250.0;
const RESPAWN_PRICE: f64 = 0.0;

#[derive(Clone)]
pub struct MedicalService<R> {
    bank: BankService<R>,
}

impl<R> MedicalService<R>
where
    R: BankRepository,
{
    pub const fn new(bank: BankService<R>) -> Self {
        Self { bank }
    }

    pub fn record_respawn(&self, uid: &str) -> Result<ServiceReceipt, ServiceError> {
        self.record_respawn_with_fee(uid, default_respawn_price())
    }

    pub fn record_respawn_with_fee(
        &self,
        uid: &str,
        respawn_price: Money,
    ) -> Result<ServiceReceipt, ServiceError> {
        validate_uid(uid)?;
        if respawn_price.cents() < 0 {
            return Err(ServiceError::InvalidAmount);
        }
        if respawn_price.is_positive() {
            self.bank.withdraw_from_account(uid, respawn_price)?;
        }

        Ok(ServiceReceipt {
            uid: uid.to_string(),
            kind: ServiceKind::MedicalRespawn,
            amount: respawn_price.to_amount(),
            description: "player respawned after death".to_string(),
        })
    }

    pub fn full_heal(&self, uid: &str) -> Result<ServiceReceipt, ServiceError> {
        self.full_heal_with_fee(uid, default_full_heal_price())
    }

    pub fn full_heal_with_fee(
        &self,
        uid: &str,
        full_heal_price: Money,
    ) -> Result<ServiceReceipt, ServiceError> {
        validate_uid(uid)?;
        if full_heal_price.cents() < 0 {
            return Err(ServiceError::InvalidAmount);
        }
        if full_heal_price.is_positive() {
            self.bank.withdraw_from_account(uid, full_heal_price)?;
        }

        Ok(ServiceReceipt {
            uid: uid.to_string(),
            kind: ServiceKind::MedicalHeal,
            amount: full_heal_price.to_amount(),
            description: "player fully healed".to_string(),
        })
    }
}

fn validate_uid(uid: &str) -> Result<(), ServiceError> {
    if uid.trim().is_empty() {
        return Err(ServiceError::InvalidUid);
    }
    Ok(())
}

fn default_full_heal_price() -> Money {
    Money::from_major(FULL_HEAL_PRICE).unwrap_or(Money::ZERO)
}

fn default_respawn_price() -> Money {
    Money::from_major(RESPAWN_PRICE).unwrap_or(Money::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryBankRepository;

    #[test]
    fn full_heal_charges_bank_account() {
        let bank = BankService::new(InMemoryBankRepository::new());
        bank.init_player_account("steam:local-dev", "0.00", "1000.00")
            .expect("account should be created");
        let service = MedicalService::new(bank.clone());

        let receipt = service
            .full_heal("steam:local-dev")
            .expect("heal should complete");

        assert_eq!(receipt.amount.as_str(), "250.00");
        let account = bank
            .get_account("steam:local-dev")
            .expect("account lookup should succeed")
            .expect("account should exist");
        assert_eq!(account.account.balance.as_str(), "750.00");
    }

    #[test]
    fn respawn_is_recorded_without_charge() {
        let service = MedicalService::new(BankService::new(InMemoryBankRepository::new()));

        let receipt = service
            .record_respawn("steam:local-dev")
            .expect("respawn should record");

        assert_eq!(receipt.amount.as_str(), "0.00");
        assert_eq!(receipt.kind, ServiceKind::MedicalRespawn);
    }

    #[test]
    fn respawn_can_charge_configured_fee() {
        let bank = BankService::new(InMemoryBankRepository::new());
        bank.init_player_account("steam:local-dev", "0.00", "1000.00")
            .expect("account should be created");
        let service = MedicalService::new(bank.clone());

        let receipt = service
            .record_respawn_with_fee("steam:local-dev", Money::from_cents(10000))
            .expect("respawn should charge");

        assert_eq!(receipt.amount.as_str(), "100.00");
        let account = bank
            .get_account("steam:local-dev")
            .expect("account lookup should succeed")
            .expect("account should exist");
        assert_eq!(account.account.balance.as_str(), "900.00");
    }
}
