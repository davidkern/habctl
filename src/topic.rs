//! Implements `TopicServer` which broadcasts typed messages to all
//! agents which have joined the topic.

use actix::prelude::*;
use std::collections::HashMap;
use std::any::{TypeId, Any};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Topic<M>
where
    M: Message + Send,
    M::Result: Send
{
    recipients: Vec<Recipient<M>>,
}

impl<M> Topic<M>
where
    M: Message + Send,
    M::Result: Send
{
    fn new() -> Self {
        Self {
            recipients: Vec::new(),
        }
    }

    fn join(&mut self, recipient: Recipient<M>) {
        self.recipients.push(recipient);
    }

    fn leave(&mut self, recipient: Recipient<M>) {
        self.recipients.retain(|r| *r != recipient);
    }
}

#[derive(Debug, Default)]
pub struct TopicServer {
    topics: HashMap<TypeId, Box<dyn Any>>,
}

impl TopicServer {
    pub fn new() -> Self {
        Self {
            topics: HashMap::new(),
        }
    }
}

impl Actor for TopicServer {
    type Context = actix::Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join<M>
where
    M: Message + Send,
    M::Result: Send
{
    recipient: Recipient<M>,
}

impl<M> Join<M>
where
    M: 'static + Message + Send,
    M::Result: Send
{
    pub fn new(recipient: Recipient<M>) -> Self {
        Self {
            recipient,
        }
    }
}

impl<M> Handler<Join<M>> for TopicServer
where
    M: 'static + Message + Send,
    M::Result: Send
{
    type Result = ();

    fn handle(&mut self, msg: Join<M>, _: &mut Context<Self>) -> Self::Result {
        let any_topic = self.topics.entry(TypeId::of::<M>()).or_insert_with(|| Box::new(Topic::<M>::new()));
        if let Some(topic) = any_topic.downcast_mut::<Topic<M>>() {
            topic.join(msg.recipient);
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Leave<M>
    where
        M: Message + Send,
        M::Result: Send
{
    recipient: Recipient<M>,
}

impl<M> Leave<M>
    where
        M: 'static + Message + Send,
        M::Result: Send
{
    pub fn new(recipient: Recipient<M>) -> Self {
        Self {
            recipient,
        }
    }
}

impl<M> Handler<Leave<M>> for TopicServer
    where
        M: 'static + Message + Send,
        M::Result: Send
{
    type Result = ();

    fn handle(&mut self, msg: Leave<M>, _: &mut Context<Self>) -> Self::Result {
        let any_topic = self.topics.entry(TypeId::of::<M>()).or_insert_with(|| Box::new(Topic::<M>::new()));
        if let Some(topic) = any_topic.downcast_mut::<Topic<M>>() {
            topic.leave(msg.recipient);
        }
    }
}
