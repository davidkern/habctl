//! Loads configuration from multiple sources

use anyhow::{Result, Error};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::collections::HashMap;


static INSTANCE: OnceCell<Config> = OnceCell::new();

#[derive(Deserialize, Debug)]
pub struct Config {
    name: String,
    devices: HashMap<String, Device>,
}

#[derive(Deserialize, Debug)]
pub struct Device {
    path: String,
    protocol: Protocol,
}

#[derive(Deserialize, Debug)]
pub enum Protocol {
    VictronVEDirect,
    VictronMk3,
}

impl Config {
    /// Loads the global config (may only be used once)
    pub fn load() -> Result<()> {
        let config_file = std::fs::read_to_string("habctl.toml")?;
        let config = toml::from_str(&config_file)?;

        INSTANCE
            .set(config)
            .map_err(|_| Error::msg("already loaded"))?;

        Ok(())
    }

    /// Gets the global config
    pub fn get() -> &'static Config {
        INSTANCE.get().expect("config not loaded")
    }    
}
