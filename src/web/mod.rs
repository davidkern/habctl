mod files;
mod socket;

use std::net::SocketAddr;
use warp::Filter;

pub async fn serve(addr: impl Into<SocketAddr>) {
    let routes = socket::ui_socket()
        .or(files::static_files());

    warp::serve(routes)
        .run(addr)
        .await;
}
