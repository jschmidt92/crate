use crate::models::FuelType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuelTransaction {
    pub user_id: u64,
    pub plate: String,
    pub liters: f64,
    pub fuel_type: FuelType,
    pub price_per_liter: f64,
}

impl FuelTransaction {
    pub fn total(&self) -> f64 {
        self.liters * self.price_per_liter
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub user_id: u64,
    pub amount: f64,
    pub description: String,
}
