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
            log::debug!("Icm20948 {} at {}", self.name, self.port);


            loop {
                task::block_in_place(|| {
                    self.read_imu_data();
                });
    
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }

        Ok(())
    }

    fn read_imu_data(&self) {
        let mut frame = ImuFrame::default();

        match i2c_linux::I2c::from_path(self.port.to_owned()) {
            Ok(i2c) => {
                let mut i2c = i2c;
                // i2c mode 8 (0-axis mode)
                let mut who_am_i: [u8; 1] = [0u8];
                i2c.smbus_set_slave_address(0x69, false).expect("set i2c address");
                let who_am_i = i2c.smbus_read_byte_data(0x00).unwrap();
                //let result = i2c.i2c_read_block_data(0x00, &mut who_am_i);
                frame.temperature = Some(who_am_i.into());
                *self.telemetry.lock().unwrap() = frame;
            },
            Err(e) => {
                log::error!("IMU {}: {}", self.name, e);
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
