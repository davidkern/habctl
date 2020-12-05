//! Implements a `BroadcastHandler` for receiving messages sent to
//! groups of `Actor`'s

use actix::{Actor, Handler, Message};

trait BroadcastHandler<M> :
    Handler<M>
where
    Self: Actor,
    M: Message
{
}