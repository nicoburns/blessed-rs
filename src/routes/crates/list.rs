use axum::{
    http::StatusCode,
    response::{ IntoResponse, Html },
};
use serde::{Serialize, Deserialize};
use serde_json;
use tera::Context;
use crate::templates::TERA;

pub(crate) async fn run() -> impl IntoResponse {


    // Load crate data
    let data_file_contents : &str = include_str!("../../../data/crates.json");
    // let data_file_contents = String::from_utf8(std::fs::read("data/crates.json").unwrap()).unwrap();
    let data : CrateDefinitionFile = serde_json::from_str(&data_file_contents).unwrap();
    
    // Render template
    let mut context = Context::new();
    context.insert("crate_groups", &data.crate_groups);
    let rendered = TERA.render("routes/crates/list.html", &context);

    // Handle template rendering errors
    let res  = match rendered {
        Ok(res) => (StatusCode::OK, Html(res)),
        Err(err) => {
            tracing::debug!("listening on {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Html("".to_string()))
        }
    };

    // Return response
    (StatusCode::OK, res)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Crate {
    name: String,
    notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Purpose {
    name: String,
    crates: Vec<Crate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CrateGroup {
    name: String,
    description: String,
    purposes: Vec<Purpose>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CrateDefinitionFile {
    crate_groups: Vec<CrateGroup>,
}