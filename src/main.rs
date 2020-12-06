pub mod hardware;
pub mod http;
pub mod network;
pub mod telemetry;

use tide::prelude::*;
use tide::log;
use crate::http::serve_dir::ServeDirWithIndex;

#[async_std::main]
async fn main() -> tide::Result<()> {
    // std::env::set_var("RUST_LOG", "habctl=debug");
    // env_logger::init();
    log::with_level(log::LevelFilter::Debug);

    let mut app = tide::new();
    // app.at("/").serve_dir("../habux/dist/")?;
    app.at("/").serve_dir_with_index("../habux/dist/")?;
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
