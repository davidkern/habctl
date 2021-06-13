//! Physical interfaces

pub mod config;
pub mod victron;

use anyhow::Result;
use victron::ve_direct::VeDirectMppt;

pub struct Hardware {
    mppt: Vec<VeDirectMppt>,
}

impl Hardware {
    pub async fn run(&self) -> Result<()> {

        //hardware::victron::ve_direct::ve_direct_mppt("/dev/serial/by-id/usb-VictronEnergy_BV_VE_Direct_cable_VE46V0KW-if00-port0"),
        Ok(())
    }
}

impl Default for Hardware {
    fn default() -> Self {
        let config = crate::Config::get();

        Self {
            mppt: config.hardware.mppt.iter().map(|(name, config)| VeDirectMppt::new(name, &config.path)).collect(),
        }
    }
}
