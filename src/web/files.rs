// use warp::Filter;
// use std::path::Path;
// use crate::STATIC_PATH;

// /// Serve static files from `habux` wasm-client project
// pub fn static_files() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     let static_index = warp::get()
//         .and(warp::path::end())
//         .and(warp::fs::file(Path::new(STATIC_PATH).join("index.html")));

//     let static_dir = warp::fs::dir(STATIC_PATH);

//     static_index.or(static_dir)
// }
