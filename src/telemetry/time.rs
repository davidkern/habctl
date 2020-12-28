// use crate::system::Sys;
// use tokio::time::{interval, Duration};
// use crate::telemetry::Data;
// use chrono::Utc;
// use crate::SYSTEM_TIME_INTERVAL;

// #[derive(Default)]
// pub struct Time {
// }

// impl Time {
//     pub async fn run(&self, sys: Sys) {
//         let mut interval = interval(Duration::from_secs(SYSTEM_TIME_INTERVAL));
//         loop {
//             interval.tick().await;
//             sys.telemetry.transmit(&Data::SystemTime(Utc::now())).await;
//         }
//     }
// }
