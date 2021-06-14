//! Physical interfaces

pub mod config;
pub mod victron;

use anyhow::Result;
use futures::future::try_join_all;
use victron::ve_direct::{self, VeDirectMppt};
use std::sync::Arc;
use serde::Serialize;

pub type Hardware = Arc<HardwareImpl>;

#[derive(Serialize)]
pub struct HardwareImpl {
    mppt: Vec<Arc<VeDirectMppt>>,
}

impl HardwareImpl {
    pub async fn run(&self) -> Result<Vec<()>> {
        let mut runners = Vec::new();

        for i in 0..self.mppt.len() {
            runners.push(self.mppt[i].run())
        }

        try_join_all(runners).await
    }
}

impl Default for HardwareImpl {
    fn default() -> Self {
        let config = crate::Config::get();

        let hardware = Self {
            mppt: config
                .hardware
                .mppt
                .iter()
                .map(|(name, config)| {
                    match &config.port {
                        Some(port) => ve_direct::new(name, &port),
                        None => ve_direct::loopback(name),
                    }
                })
                .collect(),
        };

        hardware
    }
}
