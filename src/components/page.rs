use std::{borrow::Cow, env::current_dir, fs::read_to_string};

use dioxus::prelude::*;
use fxhash::hash32;

use super::{TableOfContents, TocSection};

#[component]
pub fn Page(
    title: Cow<'static, str>,
    head: Option<Element>,
    footer: Option<Element>,
    children: Element,
) -> Element {

    // Register function to get hash of CSS file. Hash doesn't need to be secure as it is
    // purely to prevent the old version of the file being cached when the file it updated
    let cwd = current_dir().unwrap();
    let index_css_path = {
        let mut path = cwd.clone();
        path.push("static/index.css");
        path
    };
    let index_css_contents = read_to_string(index_css_path).unwrap();
    let index_css_hash = hash32(&index_css_contents);

    rsx! {
        div { lang: "en",
            head {
                Fragment {
                    meta { charset: "UTF-8" }
                    meta {
                        name: "viewport",
                        content: "width=device-width, initial-scale=1.0",
                    }
                    title { "{title} - Blessed.rs" }
                    link { href: "/static/normalize.css", rel: "stylesheet" }
                    link {
                        rel: "stylesheet",
                        href: "/static/index.css?{index_css_hash}",
                    }
                    link { href: "/static/fira-sans.css", rel: "stylesheet" }
                    link {
                        href: "/static/github-fork-ribbon.css",
                        rel: "stylesheet",
                    }
                    style { r#type: "text/css",
                        ".github-fork-ribbon:before {{ background-color: #333; }}"
                    }
                    {head}
                }
            }
            body {
                Fragment {
                    a {
                        "data-ribbon": "Fork me on GitHub",
                        href: "https://github.com/nicoburns/blessed-rs",
                        title: "Fork me on GitHub",
                        class: "github-fork-ribbon",
                        "Fork me on GitHub"
                    }
                    div { id: "header",
                        a { href: "/",
                            img {
                                src: "/static/rust-logo-black.svg",
                                class: "logo",
                            }
                        }
                        div { class: "page-heading-group",
                            h1 { class: "page-heading", "Blessed.rs" }
                            h2 { class: "page-subheading", "An unofficial guide to the Rust ecosystem" }
                        }
                    }
                    nav {
                        a { href: "/crates", "Crate Directory" }
                        " | "
                        a { href: "/learning-resources", "Learning Resources" }
                    }
                    div { id: "main-container", {children} }
                    {footer}
                }
            }
        }
    }
}

#[component]
pub fn LeftSidebar(toc_sections: Vec<TocSection>) -> Element {
    rsx! {
        div { id: "left-sidebar",
            TableOfContents { sections: toc_sections }
        }
    }
}

#[component]
pub fn MainContent(
    #[props(default)] prose: bool,
    toc_sections: Option<Vec<TocSection>>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "hflex",
            if let Some(toc_sections) = toc_sections {
                LeftSidebar { toc_sections }
            }
            div {
                id: "content",
                style: if prose { "max-width: 800px;max-width: min(800px, 100%)" },
                {children}
            }
        }
    }
}
