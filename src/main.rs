use axum::{
    routing::{get, post, get_service},
    Router,
    http::StatusCode,
};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};

mod routes;
mod templates;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let static_file_service = get_service(ServeDir::new("static"))
    .handle_error(|error: std::io::Error| async move {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {}", error))
    });

    // build our application with a route
    let app = Router::new()
        .route("/", get(routes::index))
        .route("/users", post(routes::users::create::run))
        .route("/crates", get(routes::crates::list::run))
        .nest("/static", static_file_service)
        .layer(TraceLayer::new_for_http());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let port = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3333);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

