use axum::{
    routing::{get, post, get_service},
    Router,
    response::Redirect,
};
use std::net::{SocketAddr, IpAddr};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tokio::net::TcpListener;

mod routes;
mod templates;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/crates") }))
        .route("/users", post(routes::users::create::run))
        .route("/crates", get(routes::crates::list::run))
        .route("/getting-started", get(routes::getting_started::guide::run))
        .nest_service("/static", get_service(ServeDir::new("static")))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let host : IpAddr = std::env::var("HOST").ok().and_then(|h| h.parse().ok()).unwrap_or("::".parse().unwrap());
    let port = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3333);
    let addr = SocketAddr::from((host, port));
    let listener = TcpListener::bind(addr).await.unwrap();
    
    let msg = format!("Serving blessed-rs at http://{addr}").replace("[::]", "localhost");
    println!("{msg}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

