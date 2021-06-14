use warp::Filter;
use crate::hardware::Hardware;
use std::convert::Infallible;
use std::sync::Arc;

pub fn api(hardware: Arc<Hardware>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api")
        .and(warp::get())
        .and(with_hardware(hardware))
        .and_then(reply::telemetry)
}

mod reply {
    use super::*;

    pub async fn telemetry(hardware: Arc<Hardware>) -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::json(&hardware))
    }    
}


fn with_hardware(hardware: Arc<Hardware>) -> impl Filter<Extract = (Arc<Hardware>,), Error = Infallible> + Clone {
    warp::any().map(move || hardware.clone())
}