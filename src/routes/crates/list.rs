use axum::{
    http::StatusCode,
    response::{ IntoResponse, Html },
};
use tera::{Tera, Context};

use once_cell::sync::Lazy;

static TERA: Lazy<Tera> = Lazy::new(|| {
    // let mut tera = match Tera::new("*.html") {
    //     Ok(t) => t,
    //     Err(e) => {
    //         println!("Parsing error(s): {}", e);
    //         ::std::process::exit(1);
    //     }
    // };
    let mut tera = Tera::default();
    tera.add_raw_template("list.html", include_str!("list.html")).unwrap();
    tera.autoescape_on(vec![".html", ".sql"]);
    // tera.register_filter("do_nothing", do_nothing_filter);
    tera
});

pub(crate) async fn run() -> impl IntoResponse {
    let res  = match TERA.render("list.html", &Context::new()) {
        Ok(res) => (StatusCode::OK, Html(res)),
        Err(err) => {
            tracing::debug!("listening on {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Html("".to_string()))
        }
    };
    (StatusCode::OK, res)
}