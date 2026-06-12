use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    pub uid: String,
    pub name: String,
    pub loadout: serde_json::Value,
    pub position: [f64; 3],
    pub direction: f64,
    pub stance: ActorStance,
    pub rank: ActorRank,
    pub life_state: ActorLifeState,
    pub organization: String,
    pub holster: bool,
    pub schema_version: u16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Actor {
    pub const CURRENT_SCHEMA_VERSION: u16 = 1;

    pub fn from_snapshot(snapshot: ActorSnapshot) -> Self {
        let now = Utc::now();

        Self {
            uid: snapshot.uid,
            name: snapshot.name,
            loadout: snapshot.loadout,
            position: snapshot.position,
            direction: snapshot.direction,
            stance: snapshot.stance,
            rank: snapshot.rank,
            life_state: snapshot.life_state,
            organization: snapshot.organization,
            holster: snapshot.holster,
            schema_version: Self::CURRENT_SCHEMA_VERSION,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn apply_snapshot(&mut self, snapshot: ActorSnapshot) {
        self.name = snapshot.name;
        self.loadout = snapshot.loadout;
        self.position = snapshot.position;
        self.direction = snapshot.direction;
        self.stance = snapshot.stance;
        self.rank = snapshot.rank;
        self.life_state = snapshot.life_state;
        self.organization = snapshot.organization;
        self.holster = snapshot.holster;
        self.schema_version = Self::CURRENT_SCHEMA_VERSION;
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActorSnapshot {
    pub uid: String,
    pub name: String,
    #[serde(default)]
    pub loadout: serde_json::Value,
    #[serde(default)]
    pub position: [f64; 3],
    #[serde(default)]
    pub direction: f64,
    #[serde(default)]
    pub stance: ActorStance,
    #[serde(default)]
    pub rank: ActorRank,
    #[serde(default)]
    pub life_state: ActorLifeState,
    #[serde(default = "default_organization")]
    pub organization: String,
    #[serde(default = "default_holster")]
    pub holster: bool,
}

impl ActorSnapshot {
    pub fn new(uid: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            uid: uid.into(),
            name: name.into(),
            loadout: serde_json::Value::Array(Vec::new()),
            position: [0.0, 0.0, 0.0],
            direction: 0.0,
            stance: ActorStance::default(),
            rank: ActorRank::default(),
            life_state: ActorLifeState::default(),
            organization: default_organization(),
            holster: default_holster(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActorStance {
    #[default]
    Stand,
    Crouch,
    Prone,
    Undefined,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActorRank {
    #[default]
    Private,
    Corporal,
    Sergeant,
    Lieutenant,
    Captain,
    Major,
    Colonel,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActorLifeState {
    #[default]
    Healthy,
    Injured,
    Incapacitated,
    Dead,
}

fn default_organization() -> String {
    "default".to_string()
}

const fn default_holster() -> bool {
    true
}
