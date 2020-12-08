use warp::{Filter, Reply};
use futures::{FutureExt, StreamExt};
use warp::ws::{Ws, WebSocket, Message};
use tokio::sync::mpsc;

pub fn ui_socket() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("socket" / "ui")
        .and(warp::ws())
        .map(|ws: Ws| {
            ws.on_upgrade(move |socket| socket_connected(socket))
        })
}

async fn socket_connected(ws: WebSocket) {
    let (ws_tx, mut ws_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();

    // Attach mpsc channel to `ws_tx` for sending telemetry
    tokio::task::spawn(rx.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            log::error!("websocket send error: {}", e);
        }
    }));

    // Send a message
    if let Err(_disconnected) = tx.send(Ok(Message::text("\"Hello, world!\""))) {
        // do nothing
    }

    // Process received messages until disconnect
    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(m) => m,
            Err(e) => {
                log::error!("websocket error: {}", e);
                break;
            }
        };
        socket_message(&msg).await;
    }

    // Socket disconnected
    socket_disconnected().await;
}

async fn socket_disconnected() {

}

async fn socket_message(msg: &Message) {

}
