//! Physical interfaces

pub mod config;
pub mod victron;

use anyhow::Result;
use futures::future::try_join_all;
use victron::ve_direct::{self, VeDirectMppt};
use std::sync::Arc;
use serde::Serialize;

#[derive(Serialize)]
pub struct Hardware {
    mppt: Vec<Arc<VeDirectMppt>>,
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
