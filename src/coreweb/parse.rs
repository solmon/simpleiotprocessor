mod config;
use config::*;
pub mod response;
pub use response::*;
use serde_json::Value;
use std::str::FromStr;

use super::*;
use anyhow::{anyhow, Result};

//const CONFIG: &str = include_str!("../../device.conf");
const CONFIG: &str = "Sample Config";

fn get_config() -> Result<Config> {
    CONFIG.parse::<Config>()
}

#[test]
fn parse() {
    let config = get_config();
    assert!(config.is_ok())
}

#[test]
fn test_parsing() {
    let parse = std::fs::read_to_string("./device.conf").unwrap();
    let config = toml::from_str::<Config>(&parse).map_err(|e| anyhow!(e));
    assert!(config.is_ok())
}
