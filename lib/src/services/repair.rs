use crate::{
    models::{Money, ServiceKind, ServiceQuote, ServiceReceipt},
    repositories::BankRepository,
    services::BankService,
    shared::{ServiceError, validate_plate, validate_uid},
};

const FULL_REPAIR_PRICE: f64 = 2500.0;

#[derive(Clone)]
pub struct RepairService<R> {
    bank: BankService<R>,
}

impl<R> RepairService<R>
where
    R: BankRepository,
{
    pub const fn new(bank: BankService<R>) -> Self {
        Self { bank }
    }

    pub fn quote(&self, damage: f64) -> Result<ServiceQuote, ServiceError> {
        let amount = repair_total(damage)?;

        Ok(ServiceQuote {
            kind: ServiceKind::Repair,
            amount: amount.to_amount(),
            description: format!("{damage:.2} vehicle damage repair"),
        })
    }

    pub fn complete(
        &self,
        uid: &str,
        plate: &str,
        damage: f64,
    ) -> Result<ServiceReceipt, ServiceError> {
        validate_uid(uid)?;
        validate_plate(plate)?;

        let amount = repair_total(damage)?;
        self.bank.withdraw_from_account(uid, amount)?;

        Ok(ServiceReceipt {
            uid: uid.to_string(),
            kind: ServiceKind::Repair,
            amount: amount.to_amount(),
            description: format!("{damage:.2} vehicle damage repair for {plate}"),
        })
    }
}

fn repair_total(damage: f64) -> Result<Money, ServiceError> {
    if !damage.is_finite() || !(0.0..=1.0).contains(&damage) || damage <= 0.0 {
        return Err(ServiceError::InvalidDamage);
    }

    Money::from_major(damage * FULL_REPAIR_PRICE).ok_or(ServiceError::InvalidAmount)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryBankRepository;

    #[test]
    fn complete_repair_charges_by_damage_ratio() {
        let bank = BankService::new(InMemoryBankRepository::new());
        bank.init_player_account("steam:local-dev", "0.00", "2500.00")
            .expect("account should be created");
        let service = RepairService::new(bank.clone());

        let receipt = service
            .complete("steam:local-dev", "ABC123", 0.5)
            .expect("repair should complete");

        assert_eq!(receipt.amount.as_str(), "1250.00");
        let account = bank
            .get_account("steam:local-dev")
            .expect("account lookup should succeed")
            .expect("account should exist");
        assert_eq!(account.account.balance.as_str(), "1250.00");
    }
}
