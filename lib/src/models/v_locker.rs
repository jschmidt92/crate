use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentCategory {
    Items,
    Weapons,
    Magazines,
    Backpacks,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VLocker {
    #[serde(default)]
    pub items: Vec<String>,
    #[serde(default)]
    pub weapons: Vec<String>,
    #[serde(default)]
    pub magazines: Vec<String>,
    #[serde(default)]
    pub backpacks: Vec<String>,
}

impl VLocker {
    pub fn add(&mut self, category: EquipmentCategory, classnames: Vec<String>) {
        let target = match category {
            EquipmentCategory::Items => &mut self.items,
            EquipmentCategory::Weapons => &mut self.weapons,
            EquipmentCategory::Magazines => &mut self.magazines,
            EquipmentCategory::Backpacks => &mut self.backpacks,
        };

        add_unique(target, classnames);
    }

    pub fn merge(&mut self, unlocks: &Self) {
        self.add(EquipmentCategory::Items, unlocks.items.clone());
        self.add(EquipmentCategory::Weapons, unlocks.weapons.clone());
        self.add(EquipmentCategory::Magazines, unlocks.magazines.clone());
        self.add(EquipmentCategory::Backpacks, unlocks.backpacks.clone());
    }

    pub fn get(&self, category: EquipmentCategory) -> &[String] {
        match category {
            EquipmentCategory::Items => &self.items,
            EquipmentCategory::Weapons => &self.weapons,
            EquipmentCategory::Magazines => &self.magazines,
            EquipmentCategory::Backpacks => &self.backpacks,
        }
    }

    pub fn remove(&mut self, category: EquipmentCategory, classname: &str) -> Option<String> {
        let target = match category {
            EquipmentCategory::Items => &mut self.items,
            EquipmentCategory::Weapons => &mut self.weapons,
            EquipmentCategory::Magazines => &mut self.magazines,
            EquipmentCategory::Backpacks => &mut self.backpacks,
        };

        target
            .iter()
            .position(|value| value == classname)
            .map(|index| target.remove(index))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerVLocker {
    pub uid: String,
    pub unlocks: VLocker,
}

impl PlayerVLocker {
    pub fn new(uid: impl Into<String>, unlocks: VLocker) -> Self {
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
        let mut locker = VLocker::default();
        locker.add(
            EquipmentCategory::Weapons,
            vec!["hgun_P07_F".to_string(), "hgun_P07_F".to_string()],
        );

        let mut extra = VLocker::default();
        extra.weapons = vec!["hgun_P07_F".to_string(), "arifle_MX_F".to_string()];
        locker.merge(&extra);

        assert_eq!(locker.weapons, ["hgun_P07_F", "arifle_MX_F"]);
    }
}
