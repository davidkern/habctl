//! Physical interfaces

pub mod config;
pub mod victron;

use anyhow::Result;
use victron::ve_direct::VeDirectMppt;
use futures::future::try_join_all;

pub struct Hardware {
    mppt: Vec<VeDirectMppt>,
}

impl Hardware {
    pub async fn run(&self) -> Result<Vec<()>> {
        let mut runners = Vec::new();

        for i in 0..self.mppt.len() {
            runners.push(self.mppt[i].run())
        }

        try_join_all(runners).await
    }
}

impl Default for Hardware {
    fn default() -> Self {
        let config = crate::Config::get();

        let hardware = Self {
            mppt: config.hardware.mppt.iter().map(|(name, config)| VeDirectMppt::new(name, &config.path)).collect(),
        };

        hardware
    }
}
