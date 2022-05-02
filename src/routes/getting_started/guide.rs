use axum::{
    http::StatusCode,
    response::{ IntoResponse, Html },
};
use serde::{Serialize, Deserialize};
use serde_json;
use tera::Context;
use crate::templates::{ TERA, TocSection, TocSubSection };

pub(crate) async fn run() -> impl IntoResponse {

    // Load editor data
    let data_file_contents : &str = include_str!("../../../data/editors.json");
    let editors : Vec<Editor> = serde_json::from_str(&data_file_contents).unwrap();

    let toc_sections = vec![
        TocSection { name: "Installation".into(), slug: "installation".into(), subsections: vec![
            TocSubSection { name: "Rustup".into(), slug: "cargo-rustc".into() },
            TocSubSection { name: "Editor Setup".into(), slug: "editor-setup".into() },
            TocSubSection { name: "Cargo Plugins".into(), slug: "cargo-plugins".into() },
        ]},
        TocSection { name: "Learning Resources".into(), slug: "learning-resources".into(), subsections: vec![
            TocSubSection { name: "Books".into(), slug: "books".into() },
            TocSubSection { name: "Crate Discovery".into(), slug: "crate-discovery".into() },
            TocSubSection { name: "Community Updates".into(), slug: "community-updates".into() },
            TocSubSection { name: "Asking for help".into(), slug: "help".into() },
        ]}
    ];
    
    // Render template
    let mut context = Context::new();
    context.insert("editors", &editors);
    context.insert("toc_sections", &toc_sections);
    let rendered = TERA.render("routes/getting_started/guide.html", &context);

    // Handle template rendering errors
    let res  = match rendered {
        Ok(res) => (StatusCode::OK, Html(res)),
        Err(err) => {
            tracing::debug!("Error: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("<code><pre>{:#?}</pre></code>", err)))
        }
    };

    // Return response
    (StatusCode::OK, res)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Editor {
    name: String,
    url: String,
    editor_plugins: String,
    notes: String,
}