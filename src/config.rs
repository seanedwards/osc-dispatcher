use serde::{Deserialize, Serialize};

use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::DateTime;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use url::Url;

/*

*/
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LimitConfig {
    pub rate_bps: u32,
    pub timeout_secs: u32,
    pub max_subscriptions: u32,
    pub prefix_filter: String,
    #[serde(deserialize_with = "from_ts")]
    pub ignored_until: DateTime<chrono::Utc>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Limits {
    pub bind: Option<LimitConfig>,
    pub connect: Option<LimitConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SocketMode {
    Bind,
    Connect,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SocketConfig {
    pub mode: SocketMode,
    pub addr: Url,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub socket: Vec<SocketConfig>,
    pub limits: Option<Limits>,
}

pub type ConfigHandle = Arc<Config>;

pub fn read_config<P: AsRef<Path>>(path: P) -> Config {
    let file = fs::read_to_string(&path).expect("Failed to read config file.");
    let config = toml::from_str(&file).expect("Failed to parse config.");

    config
}
