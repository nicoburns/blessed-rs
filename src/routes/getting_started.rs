use std::borrow::Cow;

use dioxus::prelude::*;

use crate::components::{MainContent, Page, Section, TocSection, TocSubSection};

#[component]
pub fn GettingStartedPage() -> Element {

    let toc_sections = vec![
        TocSection {
            name: "Installation".into(),
            slug: "installation".into(),
            subsections: vec![
                TocSubSection {
                    name: "Rust Compiler".into(),
                    slug: "cargo-rustc".into(),
                },
                TocSubSection {
                    name: "Editor Setup".into(),
                    slug: "editor-setup".into(),
                },
                TocSubSection {
                    name: "Cargo Plugins".into(),
                    slug: "cargo-plugins".into(),
                },
            ],
        },
        TocSection {
            name: "Community".into(),
            slug: "community".into(),
            subsections: vec![
                TocSubSection {
                    name: "Crate Discovery".into(),
                    slug: "crate-discovery".into(),
                },
                TocSubSection {
                    name: "Community Updates".into(),
                    slug: "community-updates".into(),
                },
                TocSubSection {
                    name: "Asking for help".into(),
                    slug: "help".into(),
                },
            ],
        },
    ];

    rsx! {
        Page { title: "Getting Started Guide".into(),
            h1 { "Getting Started Guide" }
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
            MainContent { toc_sections: Some(toc_sections),
                Section {
                    section_key: "installation".into(),
                    heading: "Installation",
                    Section {
                        section_key: "installation".into(),
                        subsection_key: Cow::from("cargo-rustc"),
                        heading: "Rust Compiler",
                        div {
                            dangerous_inner_html: r#"
                                <p>
                                  The recommended way to install Rust is using <a href="https://rustup.rs/" target="_blank">rustup</a>.
                                </p>
                                <p>
                                  Rustup is an official first-party tool which allows you to install and manage versions of rustc (the rust compiler), cargo (the rust build tool), clippy (the rust linter), rustfmt (the rust code formatter) and more. It allows you to easily upgrade to the latest version of rust, and to have multiple versions of Rust concurrently.
                                </p>

                                <h5>To install Rust using rustup:</h5>
                                <ul>
                                  <li>
                                    On macOS or Linux, run:<br />
                                    <code><pre style="margin-left: 24px">curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</pre></code>
                                  </li>
                                  <li>
                                    On Windows, download and run <a href="https://win.rustup.rs/x86_64">rustup-init.exe</a>
                                  </li>
                                </ul>
                                <p>More details and documentation on rustup are available in <a href="https://rust-lang.github.io/rustup/">the rustup book</a>.</p>
                                <p style="font-size: 0.8em;">
                                  It is <strong>not</strong> recommended to install Rust through your distribution package manager. These packages are intended to compile programs packaged by the distribution, not for Rust development, and they are often so outdated that the latest versions of libraries won't compile.
                                </p>
                            "#,
                        }
                    }

                    Section {
                        section_key: "installation".into(),
                        subsection_key: Cow::from("editor-setup"),
                        heading: "Editor Setup",
                        div { dangerous_inner_html: r#""# }
                    }
                }
            }
        }
    }
}