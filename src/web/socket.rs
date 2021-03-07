use chrono::{DateTime, Utc};
use futures::{StreamExt, SinkExt};
use serde::{Serialize, Deserialize};
use tokio::time::{self, Duration};
use warp::{Filter, Reply};
use warp::ws::{Ws, WebSocket, Message};

use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub enum Data {
    Empty,
    SystemTime(DateTime<Utc>),
}

/// UI Websocket at /socket/ui
pub fn ui_socket() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("socket" / "ui")
        .and(warp::ws())
        .map(move |ws: Ws| {
            ws.on_upgrade(move |socket| socket_connected(socket))
        })
}

/// Socket has connected
async fn socket_connected(ws: WebSocket) {
    let (mut ws_send, mut ws_recv) = ws.split();

    // handle messages from the web client
    tokio::spawn(async move {
        while let Some(msg) = ws_recv.next().await {
            log::debug!("Received {:?}", msg);
        }
        log::debug!("Exiting receive task");
    });

    // periodically send telemetry until disconnected
    let mut interval = time::interval(Duration::from_millis(Config::get().web.update_interval));
    loop {
        // send telemetry, exiting handler on error
        let msg = Data::SystemTime(Utc::now());
        if let Err(e) = ws_send.send(Message::binary(bincode::serialize(&msg).unwrap())).await {
            log::debug!("Exiting send task: {:?}", e);
            break;
        }

        interval.tick().await;
    }
}
