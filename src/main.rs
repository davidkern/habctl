use warp::Filter;
use tokio_compat_02::FutureExt;

pub mod hardware;
pub mod network;
pub mod telemetry;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([0, 0, 0, 0], 8080))
        .compat() // remove when warp is tokio 0.3
        .await;
}
