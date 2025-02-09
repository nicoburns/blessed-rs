use axum::{
    http::StatusCode,
    response::{ IntoResponse, Html },
};
use tera::Context;
use crate::templates::{ TERA, TocSection, TocSubSection };

pub(crate) async fn run() -> impl IntoResponse {

    let toc_sections = vec![
        TocSection { name: "Books".into(), slug: "books".into(), subsections: vec![
            TocSubSection { name: "Introductory Books".into(), slug: "introductory-books".into() },
            TocSubSection { name: "Advanced Books".into(), slug: "advanced-books".into() },
        ]},
        TocSection { name: "Other".into(), slug: "other".into(), subsections: vec![
            TocSubSection { name: "Learning by doing".into(), slug: "by-doing".into() },
            TocSubSection { name: "Learning by example".into(), slug: "by-example".into() },
            TocSubSection { name: "Videos".into(), slug: "videos".into() },
        ]},
        TocSection { name: "Reference".into(), slug: "reference".into(), subsections: vec![
            TocSubSection { name: "Official Documentation".into(), slug: "official-docs".into() },
            TocSubSection { name: "Unofficial Documentation".into(), slug: "unofficial-docs".into() },
        ]},
        // TocSection { name: "Community".into(), slug: "community".into(), subsections: vec![
        //     TocSubSection { name: "Crate Discovery".into(), slug: "crate-discovery".into() },
        //     TocSubSection { name: "Community Updates".into(), slug: "community-updates".into() },
        //     TocSubSection { name: "Asking for help".into(), slug: "help".into() },
        // ]}
    ];
    
    // Render template
    let mut context = Context::new();
    context.insert("toc_sections", &toc_sections);
    let rendered = TERA.render("routes/learning_resources.html", &context);

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