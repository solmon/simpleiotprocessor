mod config;
use config::*;
pub mod response;
pub use response::*;
use serde_json::Value;

use super::*;
use anyhow::{anyhow, Result};

const CONFIG: &str = include_str!("../../device-only-domains.conf");

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
    let parse = std::fs::read_to_string("./device-only-domains.conf").unwrap();
    let config = toml::from_str::<Config>(&parse).map_err(|e| anyhow!(e));
    assert!(config.is_ok())
}
