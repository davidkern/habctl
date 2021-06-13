use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Hardware {
    pub mppt: HashMap<String, Mppt>,
}

#[derive(Deserialize, Debug)]
pub struct Mppt {
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub enum Protocol {
    VictronVEDirect,
    VictronMk3,
}
