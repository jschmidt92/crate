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
        self.quote_with_price(liters, fuel_type, default_price_per_liter(fuel_type))
    }

    pub fn quote_with_price(
        &self,
        liters: f64,
        fuel_type: FuelType,
        price_per_liter: Money,
    ) -> Result<ServiceQuote, ServiceError> {
        let amount = refuel_total(liters, price_per_liter)?;

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
        self.complete_with_price(
            uid,
            plate,
            liters,
            fuel_type,
            default_price_per_liter(fuel_type),
        )
    }

    pub fn complete_with_price(
        &self,
        uid: &str,
        plate: &str,
        liters: f64,
        fuel_type: FuelType,
        price_per_liter: Money,
    ) -> Result<ServiceReceipt, ServiceError> {
        validate_uid(uid)?;
        validate_plate(plate)?;

        let amount = refuel_total(liters, price_per_liter)?;
        if amount.is_positive() {
            self.bank.withdraw_from_account(uid, amount)?;
        }

        Ok(ServiceReceipt {
            uid: uid.to_string(),
            kind: ServiceKind::Refuel,
            amount: amount.to_amount(),
            description: format!("{liters:.2} liters of {fuel_type} refuel for {plate}"),
        })
    }
}

fn refuel_total(liters: f64, price_per_liter: Money) -> Result<Money, ServiceError> {
    if !liters.is_finite() || liters <= 0.0 {
        return Err(ServiceError::InvalidAmount);
    }
    if price_per_liter.cents() < 0 {
        return Err(ServiceError::InvalidAmount);
    }

    Money::from_major(liters * price_per_liter.as_major()).ok_or(ServiceError::InvalidAmount)
}

fn default_price_per_liter(fuel_type: FuelType) -> Money {
    Money::from_major(fuel_type.price_per_liter()).unwrap_or(Money::ZERO)
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

    #[test]
    fn complete_refuel_allows_zero_configured_price() {
        let service = FuelService::new(BankService::new(InMemoryBankRepository::new()));

        let receipt = service
            .complete_with_price(
                "steam:local-dev",
                "ABC123",
                10.0,
                FuelType::Regular,
                Money::ZERO,
            )
            .expect("zero-cost refuel should complete");

        assert_eq!(receipt.amount.as_str(), "0.00");
    }
}
