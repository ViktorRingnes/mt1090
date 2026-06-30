use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app_id: String,
    pub rotmappe: String,
    pub separator: String,
    pub aad: Aad,
    pub avsenderkoder: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Aad {
    pub instance: String,
    pub tenant_id: String,
    pub client_id: String,
    pub client_secret: String,
}

pub fn get() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let raw = std::fs::read_to_string("config/default.toml").expect("read default.toml");
        toml::from_str(&raw).expect("parse default.toml")
    })
}
