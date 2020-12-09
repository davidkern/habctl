use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

pub mod solar;
pub mod time;

#[derive(Serialize, Deserialize, Debug)]
pub enum Data {
    Empty,
    Datetime(DateTime<Utc>),
}

#[derive(Default)]
pub struct Telemetry {
    receivers: Receivers,
    //tx: Option<ReceiverChannel>,
}

pub type ReceiverChannel = mpsc::UnboundedSender<Vec<u8>>;

pub struct ReceiverIdentity(usize);

impl Telemetry {
    pub async fn attach_receiver(&self, channel: ReceiverChannel) -> ReceiverIdentity {
        let id = NEXT_RECEIVER_ID.fetch_add(1, Ordering::Relaxed);
        let receiver = Receiver::new(channel);

        self.receivers.write().await.insert(id, receiver);

        log::debug!("Attached telemetry receiver #{}", id);

        ReceiverIdentity(id)
    }

    pub async fn detach_receiver(&self, identity: ReceiverIdentity) {
        self.receivers.write().await.remove(&identity.0);

        log::debug!("Detached telemetry receiver #{}", identity.0);
    }

    pub async fn transmit(&self, data: &Data) {
        log::debug!("transmitting telemetry: {:?}", data);

        let serialized = bincode::serialize(data).unwrap();

        for (id, receiver) in self.receivers.read().await.iter() {
            if let Err(e) = receiver.channel.send(serialized.to_owned()) {
                log::error!("failed transmitting telemetry to receiver #{}: {}", id, e);
            }
        }
    }
}

static NEXT_RECEIVER_ID: AtomicUsize = AtomicUsize::new(1);

struct Receiver {
    channel: ReceiverChannel,
}

impl Receiver {
    fn new(channel: ReceiverChannel) -> Self {
        Receiver {
            channel,
        }
    }
}

type Receivers = Arc<RwLock<HashMap<usize, Receiver>>>;
