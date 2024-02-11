use warp::Filter;

// This function now becomes public and returns our route.
pub fn hello_world_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| warp::reply::html("Hello, World!"))
}