use actix::{Actor, Context, Handler, Message, SystemService, Supervised, Recipient};
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait BroadcastMessage: 'static + Message<Result=()> + Send + Any + Copy {}

#[derive(Debug)]
struct Topic<MSG: BroadcastMessage>{
    recipients: Vec<Recipient<MSG>>,
}

impl<MSG: BroadcastMessage> Topic<MSG> {
    fn new() -> Self {
        Self {
            recipients: Vec::new(),
        }
    }
}

impl<MSG: BroadcastMessage> Topic<MSG> {
    fn join(&mut self, recipient: Recipient<MSG>) {
        self.recipients.push(recipient);
    }

    fn leave(&mut self, recipient: Recipient<MSG>) {
        self.recipients.retain(|r| *r != recipient);
    }

    fn send(&mut self, msg: MSG) {
        for recipient in &self.recipients {
            recipient.do_send(msg).unwrap();
        }
    }
}

pub struct Broadcast {
    topics: HashMap<TypeId, Box<dyn Any>>,
}

impl Default for Broadcast {
    fn default() -> Self {
        Self {
            topics: HashMap::new(),
        }
    }
}

impl Broadcast {
    pub fn join<MSG: BroadcastMessage>(recipient: Recipient<MSG>) {
        debug!("join called");
        Broadcast::from_registry().do_send(Join(recipient));
    }

    pub fn leave<MSG: BroadcastMessage>(recipient: Recipient<MSG>) {
        Broadcast::from_registry().do_send(Leave(recipient));
    }

    pub fn send<MSG: BroadcastMessage>(msg: MSG) {
        Broadcast::from_registry().do_send(SendMessage(msg));
    }

    fn get_or_insert_topic<MSG: BroadcastMessage>(&mut self) -> &mut Topic<MSG> {
        let any = self.topics.entry(TypeId::of::<MSG>())
            .or_insert_with(|| Box::new(Topic::<MSG>::new()));
        any.downcast_mut().unwrap()
    }
}

impl Actor for Broadcast {
    type Context = Context<Self>;
}

impl Supervised for Broadcast { }

impl SystemService for Broadcast {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        debug!("Broadcast service started");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct Join<MSG: BroadcastMessage>(Recipient<MSG>);

impl<MSG: BroadcastMessage> Handler<Join<MSG>> for Broadcast
{
    type Result = ();

    fn handle(&mut self, msg: Join<MSG>, _ctx: &mut Context<Self>) {
        self.get_or_insert_topic::<MSG>().join(msg.0);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct Leave<MSG: BroadcastMessage>(Recipient<MSG>);

impl<MSG: BroadcastMessage> Handler<Leave<MSG>> for Broadcast
{
    type Result = ();

    fn handle(&mut self, msg: Leave<MSG>, _ctx: &mut Context<Self>) {
        self.get_or_insert_topic::<MSG>().leave(msg.0);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct SendMessage<MSG: BroadcastMessage>(MSG);

impl<MSG: BroadcastMessage> Handler<SendMessage<MSG>> for Broadcast
{
    type Result = ();

    fn handle(&mut self, msg: SendMessage<MSG>, _ctx: &mut Context<Self>) {
        self.get_or_insert_topic::<MSG>().send(msg.0);
    }
}
