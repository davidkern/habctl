use warp::Filter;
use crate::hardware::Hardware;
use std::convert::Infallible;

pub fn api(hardware: Hardware) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api")
        .and(warp::get())
        .and(with_hardware(hardware))
        .and_then(reply::telemetry)
}

mod reply {
    use super::*;

    pub async fn telemetry(hardware: Hardware) -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::json(&hardware))
    }    
}


fn with_hardware(hardware: Hardware) -> impl Filter<Extract = (Hardware,), Error = Infallible> + Clone {
    warp::any().map(move || hardware.clone())
}