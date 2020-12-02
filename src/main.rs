use actix_web::{web, App, Error, HttpServer, middleware, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix_files::Files;
use actix::{Actor, StreamHandler};

pub mod phy;

#[cfg(test)]
mod test;  // Test fixtures

/// Actor handling client websocket
struct ClientSocket;

impl Actor for ClientSocket {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

/// handle websocket handshake and spawn `ClientSocket` actor
async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(ClientSocket {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(ws_index)))
            // static files
            .service(Files::new("/", "../habux/static/").index_file("index.html"))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
