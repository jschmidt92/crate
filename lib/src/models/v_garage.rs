use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleCategory {
    Cars,
    Armor,
    Helis,
    Planes,
    Naval,
    Other,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VGarage {
    #[serde(default)]
    pub cars: Vec<String>,
    #[serde(default)]
    pub armor: Vec<String>,
    #[serde(default)]
    pub helis: Vec<String>,
    #[serde(default)]
    pub planes: Vec<String>,
    #[serde(default)]
    pub naval: Vec<String>,
    #[serde(default)]
    pub other: Vec<String>,
}

impl VGarage {
    pub fn add(&mut self, category: VehicleCategory, classnames: Vec<String>) {
        let target = match category {
            VehicleCategory::Cars => &mut self.cars,
            VehicleCategory::Armor => &mut self.armor,
            VehicleCategory::Helis => &mut self.helis,
            VehicleCategory::Planes => &mut self.planes,
            VehicleCategory::Naval => &mut self.naval,
            VehicleCategory::Other => &mut self.other,
        };

        add_unique(target, classnames);
    }

    pub fn merge(&mut self, unlocks: &Self) {
        self.add(VehicleCategory::Cars, unlocks.cars.clone());
        self.add(VehicleCategory::Armor, unlocks.armor.clone());
        self.add(VehicleCategory::Helis, unlocks.helis.clone());
        self.add(VehicleCategory::Planes, unlocks.planes.clone());
        self.add(VehicleCategory::Naval, unlocks.naval.clone());
        self.add(VehicleCategory::Other, unlocks.other.clone());
    }

    pub fn get(&self, category: VehicleCategory) -> &[String] {
        match category {
            VehicleCategory::Cars => &self.cars,
            VehicleCategory::Armor => &self.armor,
            VehicleCategory::Helis => &self.helis,
            VehicleCategory::Planes => &self.planes,
            VehicleCategory::Naval => &self.naval,
            VehicleCategory::Other => &self.other,
        }
    }

    pub fn remove(&mut self, category: VehicleCategory, classname: &str) -> Option<String> {
        let target = match category {
            VehicleCategory::Cars => &mut self.cars,
            VehicleCategory::Armor => &mut self.armor,
            VehicleCategory::Helis => &mut self.helis,
            VehicleCategory::Planes => &mut self.planes,
            VehicleCategory::Naval => &mut self.naval,
            VehicleCategory::Other => &mut self.other,
        };

        target
            .iter()
            .position(|value| value == classname)
            .map(|index| target.remove(index))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerVGarage {
    pub uid: String,
    pub unlocks: VGarage,
}

impl PlayerVGarage {
    pub fn new(uid: impl Into<String>, unlocks: VGarage) -> Self {
        Self {
            uid: uid.into(),
            unlocks,
        }
    }
}

fn add_unique(target: &mut Vec<String>, classnames: Vec<String>) {
    for classname in classnames {
        if !classname.is_empty() && !target.contains(&classname) {
            target.push(classname);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_adds_unique_unlocks() {
        let mut garage = VGarage::default();
        garage.add(
            VehicleCategory::Cars,
            vec!["B_Quadbike_01_F".to_string(), "B_Quadbike_01_F".to_string()],
        );

        let mut extra = VGarage::default();
        extra.cars = vec!["B_Quadbike_01_F".to_string(), "C_Offroad_01_F".to_string()];
        garage.merge(&extra);

        assert_eq!(garage.cars, ["B_Quadbike_01_F", "C_Offroad_01_F"]);
    }
}
