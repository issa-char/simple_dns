use std::path::PathBuf;

use serde_derive::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ServerSettings {
    pub port:u16,
    pub bind:String,
}

#[derive(Clone, Deserialize)]
pub struct MirrorSettings {
    pub enabled: bool,
    pub server: String,
}

#[derive(Clone, Deserialize)]
pub struct RuleSettings {
    pub load_as: String,
    pub path: String,
}

#[derive(Clone, Deserialize)]
pub struct logSettings {
    pub level: String,
    pub save_as: String,
    pub path: String,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server: ServerSettings,
    pub mirror: MirrorSettings,
    pub rules: Vec<RulesSettings>,
    pub logs: LogSettings,
}


#[derive(Clone, Deserialize)]
pub fn load_config(path: PathBuf) -> Config {
    let config = std::fs::read_to_string(path).unwrap();
    toml::from_str(&config).unwrap()
}


#[derive(Clone, Deserialize)]
pub fn load_config_relative(path:&str) -> Config {
    let current_dir = std::env().current_dir().unwrap();
    let path = current_dir.join(path);
    load_config(path)
}


