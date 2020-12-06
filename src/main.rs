#[macro_use]
extern crate log;

pub mod hardware;
pub mod network;
pub mod telemetry;

fn main() {
    std::env::set_var("RUST_LOG", "habctl=debug");
    env_logger::init();
}
