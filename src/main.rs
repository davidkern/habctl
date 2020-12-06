#[macro_use]
extern crate log;

pub mod broadcast;
pub mod hardware;
pub mod network;
pub mod telemetry;

use actix_web::{web, App, Error, HttpServer, middleware, HttpRequest, HttpResponse};
use actix_files::Files;
use network::socket::UISocket;
use crate::broadcast::Broadcast;
use actix::SystemService;
use crate::telemetry::solar::SolarTelemetryService;

/// handle websocket handshake and spawn `ClientSocket` actor
async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = UISocket::start(req, stream);
    debug!("ws_index response: {:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "habctl=debug,actix_server=info,actix_web=info");
    env_logger::init();

    debug!("creating HttpServer");
    HttpServer::new(|| {
        // ensures services are started at launch
        Broadcast::from_registry();
        SolarTelemetryService::from_registry();

        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // user interface websocket route
            .service(web::resource("/socket/ui").route(web::get().to(ws_index)))
            // static files
            .service(Files::new("/", "../habux/dist/").index_file("index.html"))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
