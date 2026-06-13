use serde::Deserialize;
use std::{env, fs, path::PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct ForgeConfig {
    #[serde(default)]
    pub database: DatabaseConfig,
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    #[serde(default = "default_namespace")]
    pub namespace: String,
    #[serde(default = "default_database")]
    pub database: String,
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default = "default_password")]
    pub password: String,
    #[serde(default = "default_channel_capacity")]
    pub channel_capacity: usize,
    #[serde(default = "default_reconnect_initial_ms")]
    pub reconnect_initial_ms: u64,
    #[serde(default = "default_reconnect_max_ms")]
    pub reconnect_max_ms: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: default_endpoint(),
            namespace: default_namespace(),
            database: default_database(),
            username: default_username(),
            password: default_password(),
            channel_capacity: default_channel_capacity(),
            reconnect_initial_ms: default_reconnect_initial_ms(),
            reconnect_max_ms: default_reconnect_max_ms(),
        }
    }
}

pub fn load() -> ForgeConfig {
    let path = path();
    let Ok(contents) = fs::read_to_string(&path) else {
        return ForgeConfig::default();
    };

    toml::from_str(&contents).unwrap_or_default()
}

pub fn path() -> PathBuf {
    env::var_os("FORGE_SERVER_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("config.toml"))
}

fn default_endpoint() -> String {
    "127.0.0.1:8000".to_string()
}

fn default_namespace() -> String {
    "forge".to_string()
}

fn default_database() -> String {
    "forge".to_string()
}

fn default_username() -> String {
    "root".to_string()
}

fn default_password() -> String {
    "root".to_string()
}

const fn default_channel_capacity() -> usize {
    1024
}

const fn default_reconnect_initial_ms() -> u64 {
    250
}

const fn default_reconnect_max_ms() -> u64 {
    5000
}
