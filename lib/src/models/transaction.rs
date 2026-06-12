use crate::models::FuelType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuelTransaction {
    pub uid: String,
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
    pub uid: String,
    pub amount: f64,
    pub description: String,
}
