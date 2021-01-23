//! Loads configuration from multiple sources

use anyhow::{Result, Error};
use once_cell::sync::OnceCell;
use serde::Deserialize;


static INSTANCE: OnceCell<Config> = OnceCell::new();

/// Global configuration
#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(skip, default)]
    pub build: Build,
    pub hardware: crate::hardware::config::Hardware,
    pub web: crate::web::config::Web,
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

/// Configuration captured at build time
#[derive(Debug)]
pub struct Build {
    pub name: &'static str,
    pub version: &'static str,
}

impl Default for Build {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}
