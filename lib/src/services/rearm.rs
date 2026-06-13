use crate::{
    models::{Money, ServiceKind, ServiceQuote, ServiceReceipt},
    repositories::BankRepository,
    services::BankService,
    shared::{ServiceError, validate_plate, validate_uid},
};

const REARM_UNIT_PRICE: f64 = 75.0;

#[derive(Clone)]
pub struct RearmService<R> {
    bank: BankService<R>,
}

impl<R> RearmService<R>
where
    R: BankRepository,
{
    pub const fn new(bank: BankService<R>) -> Self {
        Self { bank }
    }

    pub fn quote(&self, units: u32) -> Result<ServiceQuote, ServiceError> {
        let amount = rearm_total(units)?;

        Ok(ServiceQuote {
            kind: ServiceKind::Rearm,
            amount: amount.to_amount(),
            description: format!("{units} rearm units"),
        })
    }

    pub fn complete(
        &self,
        uid: &str,
        plate: &str,
        units: u32,
    ) -> Result<ServiceReceipt, ServiceError> {
        validate_uid(uid)?;
        validate_plate(plate)?;

        let amount = rearm_total(units)?;
        self.bank.withdraw_from_account(uid, amount)?;

        Ok(ServiceReceipt {
            uid: uid.to_string(),
            kind: ServiceKind::Rearm,
            amount: amount.to_amount(),
            description: format!("{units} rearm units for {plate}"),
        })
    }
}

fn rearm_total(units: u32) -> Result<Money, ServiceError> {
    if units == 0 {
        return Err(ServiceError::InvalidAmount);
    }

    Money::from_major(f64::from(units) * REARM_UNIT_PRICE).ok_or(ServiceError::InvalidAmount)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryBankRepository;

    #[test]
    fn complete_rearm_charges_by_unit_count() {
        let bank = BankService::new(InMemoryBankRepository::new());
        bank.init_player_account("steam:local-dev", "0.00", "1000.00")
            .expect("account should be created");
        let service = RearmService::new(bank.clone());

        let receipt = service
            .complete("steam:local-dev", "ABC123", 4)
            .expect("rearm should complete");

        assert_eq!(receipt.amount.as_str(), "300.00");
        let account = bank
            .get_account("steam:local-dev")
            .expect("account lookup should succeed")
            .expect("account should exist");
        assert_eq!(account.account.balance.as_str(), "700.00");
    }
}
