//! System state

use crate::telemetry::Telemetry;

pub type Sys = &'static System;

#[derive(Default)]
pub struct System {
    pub telemetry: Telemetry,
}
