//! Implements `TopicServer` which broadcasts typed messages to all
//! agents which have joined the topic.

pub struct TopicServer;

impl TopicServer {
    pub fn new() -> Self {
        TopicServer { }
    }
}
