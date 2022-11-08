use axum::{
    http::StatusCode,
    response::{ IntoResponse, Html },
};
use serde::{Serialize, Deserialize};
use serde_json;
use tera::Context;
use crate::templates::{ TERA, TocSection, TocSubSection };

fn crates_to_toc(crates: &[CrateGroup]) ->  Vec<TocSection> {
    return crates.iter().map(|group| {
        TocSection {
            name: group.name.clone(),
            slug: group.slug.clone(),
            subsections: group.subgroups.iter().map(|subgroup| {
                TocSubSection {
                    name: subgroup.name.clone(),
                    slug: subgroup.slug.clone(),
                }
            }).collect()
        }
    }).collect()
}

pub(crate) async fn run() -> impl IntoResponse {


    // Load crate data
    let data_file_contents : &str = include_str!("../../../data/crates.json");
    // let data_file_contents = String::from_utf8(std::fs::read("data/crates.json").unwrap()).unwrap();
    let data : CrateDefinitionFile = serde_json::from_str(&data_file_contents).unwrap();
    
    // Render template
    let mut context = Context::new();
    context.insert("crate_groups", &data.crate_groups);
    context.insert("toc_sections", &crates_to_toc(&data.crate_groups));
    let rendered = TERA.render("routes/crates/crates-list.html", &context);

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
struct Crate {
    name: String,
    notes: Option<String>,
    link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Purpose {
    name: String,
    notes: Option<String>,
    recommendations: Option<Vec<Crate>>,
    see_also: Option<Vec<Crate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CrateSubGroup {
    slug: String,
    name: String,
    description: Option<String>,
    purposes: Vec<Purpose>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CrateGroup {
    slug: String,
    name: String,
    description: Option<String>,
    subgroups: Vec<CrateSubGroup>,
    purposes: Option<Vec<Purpose>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CrateDefinitionFile {
    crate_groups: Vec<CrateGroup>,
}