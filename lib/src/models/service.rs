use crate::models::MoneyAmount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceKind {
    Refuel,
    Repair,
    Rearm,
    MedicalRespawn,
    MedicalHeal,
}

impl std::fmt::Display for ServiceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Refuel => f.write_str("refuel"),
            Self::Repair => f.write_str("repair"),
            Self::Rearm => f.write_str("rearm"),
            Self::MedicalRespawn => f.write_str("medical_respawn"),
            Self::MedicalHeal => f.write_str("medical_heal"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceQuote {
    pub kind: ServiceKind,
    pub amount: MoneyAmount,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceReceipt {
    pub uid: String,
    pub kind: ServiceKind,
    pub amount: MoneyAmount,
    pub description: String,
}
