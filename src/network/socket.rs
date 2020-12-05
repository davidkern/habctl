use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix::{Actor, ActorContext, AsyncContext, StreamHandler, Handler};
use std::time::{Instant, Duration};

use crate::telemetry::solar::SolarTelemetry;
use crate::topic::{TopicServer, Join};
use actix_web::web::Data;
use crate::AppData;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

/// Actor handling user interface websocket
#[derive(Debug)]
pub struct UISocket {
    id: u64,
    last_heartbeat: Instant,
}

impl UISocket {
    pub fn start(props: Data<AppData>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
        match ws::start_with_addr(
            Self {
                id: 0,
                last_heartbeat: Instant::now(),
            },
            &req,
            stream) {
            Ok((addr, response)) => {
                //TODO: TopicService isn't going to work correctly here
                //props.services.topics.do_send(Join::new(addr.recipient()));

                Ok(response)
            },
            Err(e) => Err(e),
        }
    }

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        let id = self.id;
        ctx.run_interval(HEARTBEAT_INTERVAL, move |actor, ctx| {
            if Instant::now().duration_since(actor.last_heartbeat) > CLIENT_TIMEOUT {
                info!("UISocket[{}] heartbeat missed, disconnecting", id);
                ctx.stop();
            } else {
                ctx.ping(b"");
            }
        });
    }
}

impl Actor for UISocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for UISocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        debug!("UISocket[{}] => {:?}", self.id, msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg)
            },
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();
            },
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl Handler<SolarTelemetry> for UISocket {
    type Result = ();

    fn handle(
        &mut self,
        _msg: SolarTelemetry,
        _ctx: &mut Self::Context,
    ) {
    }
}