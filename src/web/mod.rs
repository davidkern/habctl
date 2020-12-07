mod files;
mod socket;

use tokio_compat_02::FutureExt as CompatFutureExt;
use std::net::SocketAddr;
use warp::Filter;

pub async fn serve(addr: impl Into<SocketAddr>) {
    let routes = socket::ui_socket()
        .or(files::static_files());

    warp::serve(routes)
        .run(addr)
        .compat() // TODO: remove when warp is tokio 0.3
        .await;
}
