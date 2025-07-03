use std::borrow::Cow;

use dioxus::prelude::*;
use dioxus_html_macro::html;
use serde::{Deserialize, Serialize};

use crate::components::{MainContent, Page, Section, TocSection, TocSubSection};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Crate {
    pub name: String,
    pub notes: Option<String>,
    pub link: Option<String>,
    pub docs: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Purpose {
    pub name: String,
    pub notes: Option<String>,
    pub recommendations: Option<Vec<Crate>>,
    pub see_also: Option<Vec<Crate>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrateSubGroup {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub purposes: Vec<Purpose>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrateGroup {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub subgroups: Vec<CrateSubGroup>,
    pub purposes: Option<Vec<Purpose>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrateDefinitionFile {
    pub crate_groups: Vec<CrateGroup>,
}

fn crates_to_toc(crates: &[CrateGroup]) -> Vec<TocSection> {
    return crates
        .iter()
        .map(|group| TocSection {
            name: group.name.clone(),
            slug: group.slug.clone(),
            subsections: group
                .subgroups
                .iter()
                .map(|subgroup| TocSubSection {
                    name: subgroup.name.clone(),
                    slug: subgroup.slug.clone(),
                })
                .collect(),
        })
        .collect();
}

static CRATE_GROUPS: GlobalSignal<Vec<CrateGroup>> = Signal::global(|| {
    // Load crate data
    let data_file_contents: &str = include_str!("../../data/crates.json");
    // let data_file_contents = String::from_utf8(std::fs::read("data/crates.json").unwrap()).unwrap();
    let data: CrateDefinitionFile = serde_json::from_str(&data_file_contents).unwrap();

    data.crate_groups
});

static TOC_SECTIONS: GlobalSignal<Vec<TocSection>> =
    Signal::global(|| crates_to_toc(&CRATE_GROUPS.read()));

#[component]
pub fn CrateListPage() -> Element {
    rsx! {
        Page { title: "Crate List".into(),
            h1 { "Recommended Crate Directory" }
            p {
                class: "introduction",
                dangerous_inner_html: r#"
                    The standard library in Rust is not "batteries included", excluding functionality like HTTP(S),
                    JSON, timezones, random numbers, and async IO. The recommended crate directory is a hand-curated guide to the
                    crates.io ecosystem, helping you choose which crates to use.
                "#,
            }
            p {
                class: "introduction",
                dangerous_inner_html: r#"
                    See also: <a href="https://crates.io" target="_blank">crates.io</a> (the official crates directory) and 
                    <a href="https://lib.rs" target="_blank">lib.rs</a> (a semi-curated directory).
                "#,
            }
            MainContent { toc_sections: Some(TOC_SECTIONS()),

                for group in CRATE_GROUPS() {

                    Section {
                        section_key: Cow::from(group.slug.clone()),
                        heading: group.name,
                        description: group.description,
                        if let Some(purposes) = group.purposes {
                            CrateTable { purposes }
                        }
                        for subgroup in group.subgroups {
                            Section {
                                section_key: Cow::from(group.slug.clone()),
                                subsection_key: Cow::from(subgroup.slug),
                                heading: subgroup.name,
                                description: subgroup.description,
                                CrateTable { purposes: subgroup.purposes }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CrateTable(purposes: Vec<Purpose>) -> Element {
    rsx! {

        table { class: "full-width fixed-layout",
            thead {
                tr {
                    th { class: "use-case-column", "Use Case" }
                    th { "Recommended Crates" }
                }
            }
            for purpose in purposes {
                tbody {
                    tr {
                        td { "{ purpose.name }" }
                        td {
                            if let Some(notes) = purpose.notes {
                                p {
                                    style: "margin: 3px 6px 6px 6px;font-style: italic;color: #666",
                                    dangerous_inner_html: notes,
                                }
                            }
                            if let Some(recommendations) = purpose.recommendations {
                                for krate in recommendations {
                                    CrateListItem { krate }
                                }
                            }
                            if let Some(see_also) = purpose.see_also {
                                details { style: "margin-top: 6px",
                                    summary { style: "cursor: pointer",
                                        b {
                                            i { "See also" }
                                        }
                                        span { style: "color: #999", " (click to open)" }
                                    }
                                    for krate in see_also {
                                        CrateListItem { krate }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CrateListItem(krate: Crate) -> Element {
    let show_doc_link = krate.docs.is_some() || krate.link.is_none();
    let main_href = krate
        .link
        .unwrap_or_else(|| format!("https://lib.rs/crates/{}", &krate.name));
    let doc_href = krate
        .docs
        .unwrap_or_else(|| format!("https://docs.rs/{}", &krate.name));

    let doc_link = show_doc_link.then(|| html!(<DocLink href={doc_href} />));

    html!(
        <p style="margin: 3px 6px;max-width: 600px">
            <CrateLink href={main_href} label={krate.name} /> {doc_link}
            <br />
            <span dangerous_inner_html={krate.notes} />
        </p>
    )
}

#[component]
fn CrateLink(href: String, label: String) -> Element {
    html!(<b><a href={href} style="text-decoration: underline" target="_blank">{label}</a></b>)
}

#[component]
fn DocLink(href: String) -> Element {
    html!(<a href={href} style="margin-left: 3px;opacity: 0.7;color: #333" target="_blank">"[docs]"</a>)
}
