//! Physical interfaces

pub mod config;
pub mod device;
pub mod imu;
pub mod victron;

use anyhow::Result;
use futures::future::try_join_all;
use imu::Icm20948;
use victron::ve_direct::VeDirectMppt;
use std::sync::Arc;
use serde::Serialize;
use device::Device;
use std::pin::Pin;
use std::future::Future;
use std::time::SystemTime;

#[derive(Serialize)]
pub struct Hardware {
    imu: Vec<Arc<Icm20948>>,
    mppt: Vec<Arc<VeDirectMppt>>,
}

impl Hardware {
    pub async fn run(&self) -> Result<Vec<Vec<()>>> {

        let mut imu_runners = Vec::new();
        for i in 0..self.imu.len() {
            imu_runners.push(self.imu[i].run())
        }

        let mut mppt_runners = Vec::new();
        for i in 0..self.mppt.len() {
            mppt_runners.push(self.mppt[i].run())
        }

        try_join_all(
            vec![
                Box::pin(try_join_all(imu_runners)) as Pin<Box<dyn Future<Output=Result<Vec<()>>>>>,
                Box::pin(try_join_all(mppt_runners))    
            ]
        ).await
    }
}

impl Default for Hardware {
    fn default() -> Self {
        let config = crate::Config::get();

        let hardware = Self {
            imu: config
                .hardware
                .imu
                .iter()
                .map(|(name, config)| {
                    match &config.port {
                        Some(port) => Icm20948::device(name, &port),
                        None => Icm20948::loopback(name),
                    }
                })
                .collect(),
            mppt: config
                .hardware
                .mppt
                .iter()
                .map(|(name, config)| {
                    match &config.port {
                        Some(port) => VeDirectMppt::device(name, &port),
                        None => VeDirectMppt::loopback(name),
                    }
                })
                .collect(),
        };

        hardware
    }
}

pub fn timestamp() -> f32 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs_f32()
}