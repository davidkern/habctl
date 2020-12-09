use crate::system::Sys;
use tokio::time::{interval, Duration};
use crate::telemetry::Data;
use chrono::Utc;

#[derive(Default)]
pub struct Time {
}

impl Time {
    pub async fn run(&self, sys: Sys) {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            sys.telemetry.transmit(&Data::Datetime(Utc::now())).await;
        }
    }
}