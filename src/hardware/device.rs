use std::sync::Arc;

pub trait Device {
    fn device(name: &str, path: &str) -> Arc<Self>;    
    fn loopback(name: &str) -> Arc<Self>;
}
