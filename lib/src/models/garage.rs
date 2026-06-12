use crate::shared::GarageError;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Garage {
    #[serde(default)]
    pub vehicles: HashMap<String, Vehicle>,
}

impl Garage {
    pub fn add_vehicle(&mut self, vehicle: Vehicle) -> Result<(), GarageError> {
        vehicle.validate()?;
        self.vehicles.insert(vehicle.plate.clone(), vehicle);
        Ok(())
    }

    pub fn remove_vehicle(&mut self, plate: &str) -> Option<Vehicle> {
        self.vehicles.remove(plate)
    }

    pub fn get_vehicle(&self, plate: &str) -> Option<&Vehicle> {
        self.vehicles.get(plate)
    }

    pub fn get_vehicle_mut(&mut self, plate: &str) -> Option<&mut Vehicle> {
        self.vehicles.get_mut(plate)
    }

    pub fn validate(&self) -> Result<(), GarageError> {
        for vehicle in self.vehicles.values() {
            vehicle.validate()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vehicle {
    pub plate: String,
    pub classname: String,
    pub fuel: f64,
    pub damage: f64,
    pub hit_points: HitPoints,
}

impl Vehicle {
    pub fn new(
        plate: impl Into<String>,
        classname: impl Into<String>,
        fuel: f64,
        damage: f64,
        hit_points: HitPoints,
    ) -> Result<Self, GarageError> {
        let vehicle = Self {
            plate: plate.into(),
            classname: classname.into(),
            fuel,
            damage,
            hit_points,
        };
        vehicle.validate()?;
        Ok(vehicle)
    }

    pub fn validate(&self) -> Result<(), GarageError> {
        if self.plate.trim().is_empty() {
            return Err(GarageError::InvalidPlate);
        }

        if self.classname.trim().is_empty() {
            return Err(GarageError::InvalidClassname);
        }

        if !(0.0..=1.0).contains(&self.fuel) {
            return Err(GarageError::InvalidFuel);
        }

        if !(0.0..=1.0).contains(&self.damage) {
            return Err(GarageError::InvalidDamage);
        }

        self.hit_points.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HitPoints {
    pub names: Vec<String>,
    pub selections: Vec<String>,
    pub values: Vec<f64>,
}

impl HitPoints {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            selections: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn validate(&self) -> Result<(), GarageError> {
        if self.names.len() != self.selections.len() || self.names.len() != self.values.len() {
            return Err(GarageError::InvalidHitPoints);
        }

        for value in &self.values {
            if !(0.0..=1.0).contains(value) {
                return Err(GarageError::InvalidHitPoints);
            }
        }

        Ok(())
    }

    fn normalize_legacy_fields(&mut self) {
        if self.names.is_empty()
            && !self.selections.is_empty()
            && self.selections.len() == self.values.len()
        {
            self.names = self.selections.clone();
        }

        if self.selections.is_empty()
            && !self.names.is_empty()
            && self.names.len() == self.values.len()
        {
            self.selections = self.names.clone();
        }
    }
}

impl Default for HitPoints {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize)]
struct HitPointsWire {
    #[serde(default)]
    names: Vec<String>,
    #[serde(default)]
    selections: Vec<String>,
    #[serde(default)]
    values: Vec<f64>,
}

impl<'de> Deserialize<'de> for HitPoints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = HitPointsWire::deserialize(deserializer)?;
        let mut hit_points = Self {
            names: wire.names,
            selections: wire.selections,
            values: wire.values,
        };
        hit_points.normalize_legacy_fields();
        Ok(hit_points)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerGarage {
    pub uid: String,
    pub garage: Garage,
}

impl PlayerGarage {
    pub fn new(uid: impl Into<String>) -> Self {
        Self {
            uid: uid.into(),
            garage: Garage::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garage_adds_vehicle_by_plate() {
        let mut garage = Garage::default();
        let vehicle = Vehicle::new("ABC123", "C_Offroad_01_F", 1.0, 0.0, HitPoints::default())
            .expect("vehicle should be valid");

        garage
            .add_vehicle(vehicle)
            .expect("vehicle should be added");

        assert!(garage.get_vehicle("ABC123").is_some());
    }

    #[test]
    fn hit_points_accept_legacy_missing_names() {
        let hit_points: HitPoints =
            serde_json::from_str(r#"{"selections":["engine"],"values":[0.35]}"#)
                .expect("legacy hit points should deserialize");

        assert_eq!(hit_points.names, ["engine"]);
        assert_eq!(hit_points.selections, ["engine"]);
    }

    #[test]
    fn vehicle_rejects_invalid_damage() {
        assert_eq!(
            Vehicle::new("ABC123", "C_Offroad_01_F", 1.0, 1.5, HitPoints::default())
                .expect_err("damage should be invalid"),
            GarageError::InvalidDamage
        );
    }
}
