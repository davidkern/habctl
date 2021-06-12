use crate::config::Config;
use std::path::Path;
use warp::Filter;

/// Serve static files from `habux` wasm-client project
pub fn static_files() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let static_path = &Config::get().web.static_path;

    let static_index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(Path::new(static_path).join("index.html")));

    let static_dir = warp::fs::dir(static_path);

    static_index.or(static_dir)
}
