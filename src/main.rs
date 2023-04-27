#[macro_use]
extern crate tracing;

use axum::handler::HandlerWithoutStateExt as _;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 5002));

    let serve_dir = ServeDir::new("assets").not_found_service(handle_404.into_service());

    // Try to create the assets folder for convenience
    let _ = std::fs::create_dir_all("./assets");

    let app = Router::new()
        .route("/", get(index))
        .fallback_service(serve_dir);

    let server = axum::Server::bind(&addr);

    info!("Listening on {addr}");

    server
        .serve(app.into_make_service())
        .await
        .expect("server error");
}

async fn index() -> &'static str {
    r"Hello ＼\ ٩( ᐛ )و /／"
}

async fn handle_404() -> (StatusCode, &'static str) {
    (
        StatusCode::NOT_FOUND,
        "Ressource nowhere to be found… ( ´•ω•` )ｼｮﾎﾞｰﾝ",
    )
}
