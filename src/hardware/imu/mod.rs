use anyhow::Result;
use tokio::time::{sleep, Duration};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use crate::hardware::device::Device;
use std::time::SystemTime;
use tokio::task;
use nalgebra as na;
use std::os::unix::io::AsRawFd;

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
            task::block_in_place(|| {
                self.stream_i2c_data();
            })
        }

        Ok(())
    }

    fn stream_i2c_data(&self) {
        let mut frame = ImuFrame::default();

        if let Ok(i2c) = i2c_linux::I2c::from_path(self.port.to_owned()) {
            let mut i2c = i2c;
            // i2c mode 8 (0-axis mode)
            let mut who_am_i: [u8; 1] = [0u8];
            let result = i2c.i2c_read_block_data(0x00, &mut who_am_i);
            frame.temperature = Some(who_am_i[0].into());
            *self.telemetry.lock().unwrap() = frame;

            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }

    }    
}


#[derive(Default, Clone, Debug, Serialize)]
pub struct ImuFrame {
    timestamp: Option<SystemTime>,

    /// Rotation rate in degress per second (max 2000 dps)
    gyroscope: Option<na::Rotation3<f32>>,

    /// Accelerameter 3-vector in g (max 2g)
    accelerometer: Option<na::Vector3<f32>>,

    /// Magnetometer 3-vector in Tesla (max 4900 microTesla)
    magnetometer: Option<na::Vector3<f32>>,

    /// IMU temperature in deg C
    temperature: Option<u16>,
}
