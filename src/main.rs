#[macro_use]
extern crate log;

pub mod hardware;
pub mod telemetry;
pub mod topic;
pub mod network;

use actix_web::{web, App, Error, HttpServer, middleware, HttpRequest, HttpResponse};
use actix_files::Files;
use network::socket::UISocket;
use std::sync::Arc;
use crate::topic::TopicServer;
use actix_web::web::Data;

#[cfg(test)]
mod test;  // Test fixtures

/// handle websocket handshake and spawn `ClientSocket` actor
async fn ws_index(props: Data<AppData>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = UISocket::start(props, req, stream);
    debug!("ws_index response: {:?}", resp);
    resp
}

pub struct AppServices {
    topics: TopicServer,
}

impl AppServices {
    fn new() -> Self {
        Self {
            topics: TopicServer::new(),
        }
    }
}

#[derive(Clone)]
pub struct AppData {
    services: Arc<AppServices>,
}

impl AppData {
    fn new() -> Self {
        Self {
            services: Arc::new(AppServices::new()),
        }
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "habctl=debug,actix_server=info,actix_web=info");
    env_logger::init();

    debug!("creating HttpServer");
    HttpServer::new(|| {
        App::new()
            // app data
            .data(AppData::new())
            // enable logger
            .wrap(middleware::Logger::default())
            // user interface websocket route
            .service(web::resource("/socket/network").route(web::get().to(ws_index)))
            // static files
            .service(Files::new("/", "../habux/dist/").index_file("index.html"))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
