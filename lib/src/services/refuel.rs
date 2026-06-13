use crate::{
    models::{FuelType, Money, ServiceKind, ServiceQuote, ServiceReceipt},
    repositories::BankRepository,
    services::BankService,
    shared::{ServiceError, validate_plate, validate_uid},
};

#[derive(Clone)]
pub struct FuelService<R> {
    bank: BankService<R>,
}

impl<R> FuelService<R>
where
    R: BankRepository,
{
    pub const fn new(bank: BankService<R>) -> Self {
        Self { bank }
    }

    pub fn price(&self, fuel_type: FuelType) -> f64 {
        fuel_type.price_per_liter()
    }

    pub fn quote(&self, liters: f64, fuel_type: FuelType) -> Result<ServiceQuote, ServiceError> {
        let amount = refuel_total(liters, fuel_type)?;

        Ok(ServiceQuote {
            kind: ServiceKind::Refuel,
            amount: amount.to_amount(),
            description: format!("{liters:.2} liters of {fuel_type} refuel"),
        })
    }

    pub fn complete(
        &self,
        uid: &str,
        plate: &str,
        liters: f64,
        fuel_type: FuelType,
    ) -> Result<ServiceReceipt, ServiceError> {
        validate_uid(uid)?;
        validate_plate(plate)?;

        let amount = refuel_total(liters, fuel_type)?;
        self.bank.withdraw_from_account(uid, amount)?;

        Ok(ServiceReceipt {
            uid: uid.to_string(),
            kind: ServiceKind::Refuel,
            amount: amount.to_amount(),
            description: format!("{liters:.2} liters of {fuel_type} refuel for {plate}"),
        })
    }
}

fn refuel_total(liters: f64, fuel_type: FuelType) -> Result<Money, ServiceError> {
    if !liters.is_finite() || liters <= 0.0 {
        return Err(ServiceError::InvalidAmount);
    }

    Money::from_major(liters * fuel_type.price_per_liter()).ok_or(ServiceError::InvalidAmount)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryBankRepository;

    #[test]
    fn complete_refuel_charges_bank_account() {
        let bank = BankService::new(InMemoryBankRepository::new());
        bank.init_player_account("steam:local-dev", "0.00", "100.00")
            .expect("account should be created");
        let service = FuelService::new(bank.clone());

        let receipt = service
            .complete("steam:local-dev", "ABC123", 10.0, FuelType::Regular)
            .expect("refuel should complete");

        assert_eq!(receipt.amount.as_str(), "10.00");
        let account = bank
            .get_account("steam:local-dev")
            .expect("account lookup should succeed")
            .expect("account should exist");
        assert_eq!(account.account.balance.as_str(), "90.00");
    }
}
