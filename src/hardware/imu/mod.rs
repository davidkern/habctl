use anyhow::Result;
use tokio::time::{sleep, Duration};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use crate::hardware::device::Device;
use std::time::SystemTime;

#[derive(Serialize)]
pub struct Icm20948 {
    loopback: bool,
    name: String,
    port: String,
    pub telemetry: Mutex<ImuFrame>,
}

impl Device for Icm20948 {
    fn device(name: &str, path: &str) -> Arc<Icm20948> {
        Arc::new(Icm20948 {
            loopback: false,
            name: name.to_owned(),
            port: path.to_owned(),
            telemetry: Mutex::default(),
        })
    }
    
    fn loopback(name: &str) -> Arc<Icm20948> {
        Arc::new(Icm20948 {
            loopback: true,
            name: name.to_owned(),
            port: String::new(),
            telemetry: Mutex::default(),
        })
    } 
}

impl Icm20948 {
    pub async fn run(&self) -> Result<()> {
        if self.loopback {
            log::debug!("Icm20948 {} is in loopback mode.", self.name);
            sleep(Duration::from_secs(600)).await;
        } else {

        }

        Ok(())
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct ImuFrame {
    timestamp: Option<SystemTime>,
}
