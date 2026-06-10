use crate::{
    models::{FuelTransaction, TransactionReceipt},
    shared::BankError,
};

pub async fn process_fuel_transaction(
    transaction: FuelTransaction,
) -> Result<TransactionReceipt, BankError> {
    let total = transaction.total();
    if !total.is_finite() || total <= 0.0 {
        return Err(BankError::InvalidAmount);
    }

    // This is the persistence boundary for the future bank repository.
    Ok(TransactionReceipt {
        user_id: transaction.user_id,
        amount: total,
        description: format!(
            "{:.2} liters of {} fuel for {}",
            transaction.liters, transaction.fuel_type, transaction.plate
        ),
    })
}
