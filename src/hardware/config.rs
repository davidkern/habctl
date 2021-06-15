use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Hardware {
    pub mppt: HashMap<String, Mppt>,
}

#[derive(Deserialize, Debug)]
pub struct Mppt {
    pub port: Option<String>,
    pub loopback: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub enum Protocol {
    VictronVEDirect,
    VictronMk3,
}
