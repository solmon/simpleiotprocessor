use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub device: Value,
    pub collections: HashMap<String, Collections>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub host: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    pub port: i32,
    pub bin: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collections {
    name: String,
    domains: HashMap<String, Domain>,
}

use anyhow::Error;
impl FromStr for Config {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        toml::from_str(s).map_err(|e| anyhow!(e))
    }
}
