use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let max_size = 15 * 1024usize.pow(2);
    let app = Router::new()
        .route(
            "/",
            post(aoe2js_api::aoe2record).layer(DefaultBodyLimit::max(max_size)),
        )
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    println!("Max size: {max_size} bytes");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // run our app with hyper, listening globally on port 3000
    let port = 3000;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("Listening on port {port}");
    axum::serve(listener, app).await.unwrap();
}
