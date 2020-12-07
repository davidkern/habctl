mod files;

use tokio_compat_02::FutureExt;
use crate::web::files::static_files;
use std::net::SocketAddr;

pub async fn serve(addr: impl Into<SocketAddr>) {
    warp::serve(static_files())
        .run(addr)
        .compat() // TODO: remove when warp is tokio 0.3
        .await;
}
