use hello_world_server::hello_world_route;

#[tokio::main]
async fn main() {
    let hello_route = hello_world_route();

    warp::serve(hello_route).run(([127, 0, 0, 1], 3030)).await;
}
