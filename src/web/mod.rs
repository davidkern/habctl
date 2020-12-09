mod files;
mod socket;

use std::net::SocketAddr;
use warp::Filter;
use crate::system::System;

pub async fn serve(sys: &'static System, addr: impl Into<SocketAddr>) {
    let routes = socket::ui_socket(sys)
        .or(files::static_files());

    warp::serve(routes)
        .run(addr)
        .await;
}
