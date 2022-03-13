use axum::{
    http::StatusCode,
    response::{ IntoResponse, Html },
};
use tera::Context;
use crate::templates::TERA;

pub(crate) async fn run() -> impl IntoResponse {
    let res  = match TERA.render("routes/crates/list.html", &Context::new()) {
        Ok(res) => (StatusCode::OK, Html(res)),
        Err(err) => {
            tracing::debug!("listening on {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Html("".to_string()))
        }
    };
    (StatusCode::OK, res)
}