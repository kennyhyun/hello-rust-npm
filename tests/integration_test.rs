use hello_world_server::hello_world_route;
use warp::http::StatusCode;
use warp::test::request;

#[tokio::test]
async fn test_hello_world() {
    let api = hello_world_route();

    let resp = request()
        .method("GET")
        .path("/")
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(resp.body(), "Hello, World!");
}
