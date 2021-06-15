// pub mod hardware;
// pub mod system;
// pub mod telemetry;
// pub mod web;

// pub const SYSTEM_TIME_INTERVAL: u64 = 60;
// pub static STATIC_PATH: &str = "../habux/dist";
// pub static WEB_LISTEN_ADDR: ([u8; 4], u16) = ([0, 0, 0, 0], 8080);

// #[tokio::main]
// async fn main() {
//     // log configuration
//     std::env::set_var("RUST_LOG", "habctl=debug,warp=debug");
//     pretty_env_logger::init();

//     log::debug!("constructing system");

//     // Allocate state up-front and freely share the reference
//     let sys: &'static mut system::System = {
//         let the_system: system::System = Default::default();
//         let boxed_system = Box::new(the_system);
//         Box::leak(boxed_system)
//     };

//     log::debug!("starting server");

//     // start services
//     tokio::join!(
//         web::serve(sys, WEB_LISTEN_ADDR),
//         sys.time.run(sys),
//     );

//     log::debug!("shutting down");
// }

#[macro_use]
extern crate bitflags;

mod config;
mod hardware;
#[cfg(test)]
mod test;
mod web;

use anyhow::Result;
use tokio::runtime::Runtime;
use std::sync::Arc;

use crate::config::Config;

fn main() -> Result<()> {
    let rt = Runtime::new()?;

    rt.block_on(async {
        // log configuration
        std::env::set_var("RUST_LOG", "habctl=debug,warp=debug");
        pretty_env_logger::init();

        log::debug!("loading config");
        Config::load()?;
        dbg!(Config::get());

        log::debug!("building hardware interfaces");
        let hardware = Arc::new(hardware::Hardware::default());

        log::debug!("starting services");
        tokio::try_join!(web::serve(Config::get().web.listen_addr, hardware.clone()), hardware.run(),)?;

        log::debug!("exiting");
        Ok(())
    })
}
