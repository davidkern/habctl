use std::time::Duration;
use actix::prelude::*;
use rand::prelude::*;
use crate::broadcast::{Broadcast, BroadcastMessage};

#[derive(Default)]
pub struct SolarTelemetryService { }

impl Actor for SolarTelemetryService {
    type Context = Context<Self>;
}

impl Supervised for SolarTelemetryService { }

impl SystemService for SolarTelemetryService {
    fn service_started(&mut self, ctx: &mut Context<Self>) {
        info!("SolarTelemetry service started");

        // ctx.run_interval(Duration::new(1, 0), |actor, ctx| {
        //     Broadcast::send(SolarTelemetry {
        //         pv0_yield: rand::random(),
        //         pv1_yield: rand::random(),
        //     });
        // });
    }
}

impl StreamHandler<SolarTelemetry> for SolarTelemetryService {
    fn handle(&mut self, item: SolarTelemetry, ctx: &mut Context<Self>) {
        Broadcast::send(item);
    }
}

#[derive(Message, Copy, Clone, Debug)]
#[rtype(result = "()")]
pub struct SolarTelemetry {
    id: &'static str,

    // pv0_yield: u16,
    // pv1_yield: u16,
}

impl BroadcastMessage for SolarTelemetry { }

