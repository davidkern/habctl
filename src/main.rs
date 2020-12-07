pub mod hardware;
pub mod network;
pub mod telemetry;
pub mod web;

pub static STATIC_PATH: &str = "../habux/dist";
pub static WEB_LISTEN_ADDR: ([u8; 4], u16) = ([0, 0, 0, 0], 8080);

#[tokio::main]
async fn main() {
    // log configuration
    std::env::set_var("RUST_LOG", "habctl=debug,warp=debug");
    pretty_env_logger::init();

    log::debug!("starting up");

    // start services
    tokio::join!(
        web::serve(WEB_LISTEN_ADDR),
    );

    log::debug!("shutting down");
}
