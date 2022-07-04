use anyhow::Result;
use tokio::time::{sleep, Duration};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use crate::hardware::device::Device;
use tokio::task;
use nalgebra as na;

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
                i2c.smbus_set_slave_address(0x69, false).expect("set i2c address");

                // power on, best available clock
                i2c.smbus_write_byte_data(0x06, 0x01).expect("power up");

                let accel_xout_h = i2c.smbus_read_byte_data(0x2d).unwrap() as f32;
                let accel_xout_l = i2c.smbus_read_byte_data(0x2e).unwrap() as f32;
                let accel_yout_h = i2c.smbus_read_byte_data(0x2f).unwrap() as f32;
                let accel_yout_l = i2c.smbus_read_byte_data(0x30).unwrap() as f32;
                let accel_zout_h = i2c.smbus_read_byte_data(0x31).unwrap() as f32;
                let accel_zout_l = i2c.smbus_read_byte_data(0x32).unwrap() as f32;
                let gyro_xout_h = i2c.smbus_read_byte_data(0x33).unwrap() as f32;
                let gyro_xout_l = i2c.smbus_read_byte_data(0x34).unwrap() as f32;
                let gyro_yout_h = i2c.smbus_read_byte_data(0x35).unwrap() as f32;
                let gyro_yout_l = i2c.smbus_read_byte_data(0x36).unwrap() as f32;
                let gyro_zout_h = i2c.smbus_read_byte_data(0x37).unwrap() as f32;
                let gyro_zout_l = i2c.smbus_read_byte_data(0x38).unwrap() as f32;
                let temp_out_h = i2c.smbus_read_byte_data(0x39).unwrap() as f32;
                let temp_out_l = i2c.smbus_read_byte_data(0x3a).unwrap() as f32;

                fn scale(hi: f32, lo: f32, s: f32) -> f32 {
                    (hi * 256.0 + lo) * s
                }

                const ACCEL_SCALE: f32 = 4.0 / 65535.0;
                let accel_x = scale(accel_xout_h, accel_xout_l, ACCEL_SCALE);
                let accel_y = scale(accel_yout_h, accel_yout_l, ACCEL_SCALE);
                let accel_z = scale(accel_zout_h, accel_zout_l, ACCEL_SCALE);

                frame.accelerometer = Some(na::Vector3::new(accel_x, accel_y, accel_z));

                const GYRO_SCALE: f32 = 250.0 / 65535.0;
                let gyro_x = scale(gyro_xout_h, gyro_xout_l, GYRO_SCALE);
                let gyro_y = scale(gyro_yout_h, gyro_yout_l, GYRO_SCALE);
                let gyro_z = scale(gyro_zout_h, gyro_zout_l, GYRO_SCALE);

                frame.gyrometer = Some(na::Vector3::new(gyro_x, gyro_y, gyro_z));

                const TEMP_SCALE: f32 = 1.0 / 333.87;
                let temp = scale(temp_out_h, temp_out_l, TEMP_SCALE) + 21.0;

                frame.temperature = Some(temp);
                frame.timestamp = Some(crate::hardware::timestamp());

                log::info!("{}: {}", self.name, frame);
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
    timestamp: Option<f32>,

    /// Rotation rate in degress per second (max 2000 dps)
    gyrometer: Option<na::Vector3<f32>>,

    /// Accelerameter 3-vector in g (max 2g)
    accelerometer: Option<na::Vector3<f32>>,

    /// Magnetometer 3-vector in Tesla (max 4900 microTesla)
    magnetometer: Option<na::Vector3<f32>>,

    /// IMU temperature in deg C
    temperature: Option<f32>,
}

impl std::fmt::Display for ImuFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let accel = if let Some(a) = self.accelerometer {
            format!("{:?}", (a.x, a.y, a.z))
        } else {
            format!("none")
        };

        let gyro = if let Some(g) = self.gyrometer {
            format!("{:?}", (g.x, g.y, g.z))
        } else {
            format!("none")
        };

        let mag = if let Some(m) = self.magnetometer {
            format!("{:?}", (m.x, m.y, m.z))
        } else {
           format!("none")
        };

        let temp = if let Some(t) = self.temperature {
            format!("{}", t)
        } else {
            format!("none")
        };

        write!(
            f,
            "ACCEL {:?} GYRO {:?} MAG {:?} TEMP {:?}",
            accel,
            gyro,
            mag,
            temp
        )
    }
}
