use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, get_service},
    Router,
};
use dioxus::prelude::*;
use dioxus_html_macro::html;
use routes::{CrateListPage, GettingStartedPage, LearningResourcesPage};
use std::{
    net::{IpAddr, SocketAddr},
    time::Instant,
};
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

mod components;
mod routes;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/crates") }))
        .route("/crates", get(|| dx_route(|| html!(<CrateListPage />))))
        .route(
            "/getting-started",
            get(|| dx_route(|| html!(<GettingStartedPage />))),
        )
        .route(
            "/learning-resources",
            get(|| dx_route(|| html!(<LearningResourcesPage />))),
        )
        .nest_service("/static", get_service(ServeDir::new("static")))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let host: IpAddr = std::env::var("HOST")
        .ok()
        .and_then(|h| h.parse().ok())
        .unwrap_or("::".parse().unwrap());
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3333);
    let addr = SocketAddr::from((host, port));
    let listener = TcpListener::bind(addr).await.unwrap();

    let msg = format!("Serving blessed-rs at http://{addr}").replace("[::]", "localhost");
    println!("{msg}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn dx_route(render_fn: fn() -> Element) -> impl IntoResponse {
    let start = Instant::now();

    let mut dom = VirtualDom::new(render_fn);
    dom.rebuild_in_place();
    let rendered = dioxus_ssr::render(&dom);
    let rendered = format!("<!DOCTYPE html><html{}", &rendered[4..]);

    println!(
        "Rendered dx in {:.2}ms",
        start.elapsed().as_micros() as f64 / 1000.0
    );

    (StatusCode::OK, Html(rendered))
}
