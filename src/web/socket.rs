use warp::{Filter, Reply};
use futures::{FutureExt, StreamExt};

pub fn ui_socket() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("socket" / "ui")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        log::error!("websocket error: {:?}", e);
                    }
                })
            })
        })
}
