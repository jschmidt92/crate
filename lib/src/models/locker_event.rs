use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockerTransferCommitted {
    pub uid: String,
    pub distinct_items: usize,
    pub total_quantity: u64,
}

impl LockerTransferCommitted {
    pub fn new(uid: impl Into<String>, distinct_items: usize, total_quantity: u64) -> Self {
        Self {
            uid: uid.into(),
            distinct_items,
            total_quantity,
        }
    }
}
