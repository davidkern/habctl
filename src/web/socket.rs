use warp::{Filter, Reply};
use warp::ws::{Ws, WebSocket};

/// UI Websocket at /socket/ui
pub fn ui_socket() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("socket" / "ui")
        .and(warp::ws())
        .map(move |ws: Ws| {
            ws.on_upgrade(move |socket| socket_connected(socket))
        })
}

/// Socket has connected
async fn socket_connected(_ws: WebSocket) {
    // send most recent telemetry
    // then new telemetry as it comes in
}
