use serde::Deserialize;
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
pub struct Hardware {
    pub devices: HashMap<String, Device>,
}

#[derive(Deserialize, Debug)]
pub struct Device {
    pub path: String,
    pub protocol: Protocol,
}

#[derive(Deserialize, Debug)]
pub enum Protocol {
    VictronVEDirect,
    VictronMk3,
}
