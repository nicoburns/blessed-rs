use std::borrow::Cow;

use dioxus::prelude::*;
use dioxus_html_macro::html;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionLevel {
    H3,
    H4,
}

#[component]
pub fn Section(
    heading: String,
    description: Option<String>,
    level: Option<SectionLevel>,
    section_key: Cow<'static, str>,
    subsection_key: Option<Cow<'static, str>>,
    children: Element,
) -> Element {
    let level = level.unwrap_or_else(|| {
        if subsection_key.is_some() {
            SectionLevel::H4
        } else {
            SectionLevel::H3
        }
    });

    rsx! {
        section {
            "data-toc-section": true,
            id: if let Some(subsection_key) = subsection_key { "section-{ section_key }-subsection-{ subsection_key }" } else { "section-{ section_key }" },
            {
                match level {
                    SectionLevel::H3 => html!(< h3 > "{heading}" </ h3 >),
                    SectionLevel::H4 => html!(< h4 > "{heading}" </ h4 >),
                }
            }

            p { class: "group-description",
                dangerous_inner_html: description
            }
            {children}
        }
    }
}
