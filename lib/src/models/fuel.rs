use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FuelType {
    Regular,
    Jeta1,
}

impl FuelType {
    pub fn price_per_liter(self) -> f64 {
        match self {
            Self::Regular => 1.0,
            Self::Jeta1 => 1.8,
        }
    }
}

impl std::fmt::Display for FuelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular => f.write_str("regular"),
            Self::Jeta1 => f.write_str("jeta1"),
        }
    }
}

impl TryFrom<&str> for FuelType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_ascii_lowercase().as_str() {
            "regular" => Ok(Self::Regular),
            "jeta1" | "jet_a1" | "jet-a1" => Ok(Self::Jeta1),
            _ => Err(()),
        }
    }
}
