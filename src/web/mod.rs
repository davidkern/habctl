pub mod config;

mod files;
mod socket;

use anyhow::Result;
use std::net::SocketAddr;
use warp::Filter;

pub async fn serve(addr: impl Into<SocketAddr>) -> Result<()> {
    let routes = socket::ui_socket().or(files::static_files());

    warp::serve(routes).run(addr).await;

    Ok(())
}
