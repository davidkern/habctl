use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use std::collections::HashMap;
use warp::ws::Message;

pub mod solar;

#[derive(Default)]
pub struct Telemetry {
    receivers: Receivers,
}

pub type ReceiverChannel = mpsc::UnboundedSender<Result<Message, warp::Error>>;

pub struct ReceiverIdentity(usize);

impl Telemetry {
    pub async fn attach_receiver(&self, channel: ReceiverChannel) -> ReceiverIdentity {
        let id = NEXT_RECEIVER_ID.fetch_add(1, Ordering::Relaxed);
        let receiver = Receiver::new(id, channel);

        self.receivers.write().await.insert(id, receiver);

        log::debug!("Attached telemetry receiver #{}", id);

        ReceiverIdentity(id)
    }

    pub async fn detach_receiver(&self, identity: ReceiverIdentity) {
        self.receivers.write().await.remove(&identity.0);

        log::debug!("Detached telemetry receiver #{}", identity.0);
    }
}

static NEXT_RECEIVER_ID: AtomicUsize = AtomicUsize::new(1);
//static RECEIVERS: Receivers = Receivers::default();

struct Receiver {
    id: usize,
    channel: ReceiverChannel,
}

impl Receiver {
    fn new(id: usize, channel: ReceiverChannel) -> Self {
        Receiver {
            id,
            channel,
        }
    }
}

type Receivers = Arc<RwLock<HashMap<usize, Receiver>>>;
