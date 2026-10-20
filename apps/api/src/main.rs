use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let cfg = config::Config::from_env().expect("Failed to load config");
    observability::init_tracing();

    let _pool = database::connect(&cfg.database_url)
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .nest("/api/v1/auth", api::routes::auth::router())
        .route("/healthz", get(api::routes::health::healthz))
        .route("/readyz", get(api::routes::health::readyz))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from((
        cfg.api_host
            .parse::<std::net::IpAddr>()
            .unwrap_or([127, 0, 0, 1].into()),
        cfg.api_port,
    ));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind");
    axum::serve(listener, app).await.expect("Server failed");
}
