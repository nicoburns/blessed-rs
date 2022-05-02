use axum::{
    routing::{get, post, get_service},
    Router,
    http::StatusCode,
    response::Redirect,
};
use std::net::{SocketAddr, IpAddr};
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
        .route("/", get(|| async { Redirect::to("/crates") }))
        .route("/users", post(routes::users::create::run))
        .route("/crates", get(routes::crates::list::run))
        .route("/getting-started", get(routes::getting_started::guide::run))
        .nest("/static", static_file_service)
        .layer(TraceLayer::new_for_http());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let host : IpAddr = std::env::var("HOST").ok().and_then(|h| h.parse().ok()).unwrap_or([127, 0, 0, 1].into());
    let port = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3333);
    let addr = SocketAddr::from((host, port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

