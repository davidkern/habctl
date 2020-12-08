use warp::{Filter, Reply};
use futures::{FutureExt, StreamExt};
use warp::ws::{Ws, WebSocket, Message};
use tokio::sync::mpsc;
use crate::system::Sys;
use crate::telemetry::ReceiverIdentity;

/// UI Websocket at /socket/ui
pub fn ui_socket(sys: Sys) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("socket" / "ui")
        .and(warp::ws())
        .map(move |ws: Ws| {
            ws.on_upgrade(move |socket| socket_connected(sys, socket))
        })
}

/// Socket has connected
async fn socket_connected(sys: Sys, ws: WebSocket) {
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
        // do nothing - `socket_disconnected` will handle cleanup
    }

    // Attach telemetry to websocket receiver
    let receiver = sys.telemetry.attach_receiver(tx).await;

    // Process received messages until disconnect
    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(m) => m,
            Err(e) => {
                log::error!("websocket error: {}", e);
                break;
            }
        };
        socket_received(&msg).await;
    }

    // Socket disconnected
    socket_disconnected(sys, receiver).await;
}

/// Socket has disconnected
async fn socket_disconnected(sys: Sys, receiver: ReceiverIdentity) {
    sys.telemetry.detach_receiver(receiver).await
}

/// Socket has received a message
async fn socket_received(_msg: &Message) {

}
