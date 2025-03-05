use axum::{
    body::Bytes,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, get_service},
    Router,
};
use dashmap::DashMap;
use dioxus::prelude::*;
use dioxus_html_macro::html;
use routes::{CrateListPage, GettingStartedPage, LearningResourcesPage};
use std::{
    any::{Any, TypeId},
    net::{IpAddr, SocketAddr},
    sync::LazyLock,
    time::{Duration, Instant},
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
        .route(
            "/crates",
            get(|| dx_route_cached(|| html!(<CrateListPage />))),
        )
        .route(
            "/getting-started",
            get(|| dx_route_cached(|| html!(<GettingStartedPage />))),
        )
        .route(
            "/learning-resources",
            get(|| dx_route_cached(|| html!(<LearningResourcesPage />))),
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

async fn dx_route_cached(render_fn: fn() -> Element) -> impl IntoResponse {
    static CACHE: LazyLock<DashMap<TypeId, Bytes>> = LazyLock::new(|| DashMap::new());

    let type_id = Any::type_id(&render_fn);

    let html = CACHE.entry(type_id).or_insert_with(|| {
        let (html, duration) = render_component(render_fn);

        let duration_millis = duration.as_micros() as f64 / 1000.0;
        println!("Rendered in {duration_millis:.2}ms",);

        Bytes::from(html)
    });

    (StatusCode::OK, Html(html.clone()))
}

#[allow(unused)]
async fn dx_route(render_fn: fn() -> Element) -> impl IntoResponse {
    let (html, duration) = render_component(render_fn);

    let duration_millis = duration.as_micros() as f64 / 1000.0;
    println!("Rendered dx in {duration_millis:.2}ms",);

    (StatusCode::OK, Html(html))
}

fn render_component(render_fn: fn() -> Element) -> (String, Duration) {
    let start = Instant::now();

    let mut dom = VirtualDom::new(render_fn);
    dom.rebuild_in_place();
    let rendered = dioxus_ssr::render(&dom);
    let html = format!("<!DOCTYPE html><html{}", &rendered[4..]);

    (html, start.elapsed())
}
