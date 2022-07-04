pub mod config;

mod api;
mod files;
mod socket;

use crate::hardware::Hardware;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use warp::Filter;

pub async fn serve(addr: impl Into<SocketAddr>, hardware: Arc<Hardware>) -> Result<()> {
    let routes = socket::ui_socket()
        .or(api::api(hardware))
        .or(files::static_files());

    warp::serve(routes).run(addr).await;

    Ok(())
}
