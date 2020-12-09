use warp::{Filter, Reply};
use futures::{StreamExt, SinkExt};
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
    let (mut ws_tx, mut ws_rx) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Send bytes received from rx as binary messages to ws_tx
    tokio::task::spawn(async move {
       while let Some(data) = rx.recv().await {
           if let Err(e) = ws_tx.send(Message::binary(data)).await {
               log::error!("websocket send error: {}", e);
           }
       }
    });

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
