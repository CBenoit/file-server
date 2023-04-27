use std::net::SocketAddr;
use tower::make::Shared;
use tower_http::services::ServeDir;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 5002));

    // serve the file in the "assets" directory under `/assets`
    let service = ServeDir::new("assets");

    hyper::Server::bind(&addr)
        .serve(Shared::new(service))
        .await
        .expect("server error");
}
