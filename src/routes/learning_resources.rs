use std::borrow::Cow;

use dioxus::prelude::*;

use crate::components::{MainContent, Page, Section, TocSection, TocSubSection};

#[component]
pub fn LearningResourcesPage() -> Element {
    let toc_sections = vec![
        TocSection {
            name: "Books".into(),
            slug: "books".into(),
            subsections: vec![
                TocSubSection {
                    name: "Introductory Books".into(),
                    slug: "introductory-books".into(),
                },
                TocSubSection {
                    name: "Advanced Books".into(),
                    slug: "advanced-books".into(),
                },
            ],
        },
        TocSection {
            name: "Other".into(),
            slug: "other".into(),
            subsections: vec![
                TocSubSection {
                    name: "Learning by doing".into(),
                    slug: "by-doing".into(),
                },
                TocSubSection {
                    name: "Learning by example".into(),
                    slug: "by-example".into(),
                },
                TocSubSection {
                    name: "Videos".into(),
                    slug: "videos".into(),
                },
            ],
        },
        TocSection {
            name: "Reference".into(),
            slug: "reference".into(),
            subsections: vec![
                TocSubSection {
                    name: "Official Documentation".into(),
                    slug: "official-docs".into(),
                },
                TocSubSection {
                    name: "Unofficial Documentation".into(),
                    slug: "unofficial-docs".into(),
                },
            ],
        },
        // TocSection { name: "Community".into(), slug: "community".into(), subsections: vec![
        //     TocSubSection { name: "Crate Discovery".into(), slug: "crate-discovery".into() },
        //     TocSubSection { name: "Community Updates".into(), slug: "community-updates".into() },
        //     TocSubSection { name: "Asking for help".into(), slug: "help".into() },
        // ]}
    ];

    rsx! {
        Page { title: "Learning Resources".into(),
            h1 { "Learning Resources" }
            p {
                class: "introduction",
                dangerous_inner_html: r#"
                     A list of books and other resources from which to learn Rust. Books which are available
                     free/online are marked. All other resources are free and online even if not marked.
                "#,
            }
            MainContent { toc_sections: Some(toc_sections),
                Section { section_key: "books".into(), heading: "Books",
                    Section {
                        section_key: "books".into(),
                        subsection_key: Cow::from("introductory-books"),
                        heading: "Introductory Books",
                        div {
                            dangerous_inner_html: r#"
                                <ul>
                                  <li>
                                    <b><a href="https://doc.rust-lang.org/book/">[Free] The Rust Programming Language</a></b>
                                    The official rust-lang book. Aimed at those with experience in another programming language but not
                                    necessarily a low-level one. This is also the best option if you are trying to learn Rust as your first programming language. Freely available online (a print version is also available). There is also an unnofficial <a href="https://jasonwalton.ca/rust-book-abridged/">abridged version</a>.
                                  </li>
                                  <li>
                                    <b><a href="https://freddiehaddad.github.io/fast-track-to-rust/">[Free] Fast Track to Rust</a></b>
                                     Introduces you to the Rust programming language by building a grep-like program with a minimal subset of the features found in the GNU grep utility. Freely available online.
                                  </li>
                                  <li>
                                    <b><a href="https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/">Programming Rust</a></b>
                                    Another book aimed at those with experience in another programming language, but not
                                    necessarily a low-level one. While the official book above has a stellar reputation, this one has a reputation for being even better,
                                    diving deeper and giving a more thorough introduction to the language. The downside being it's not free.
                                  </li>
                                  <li>
                                    <b><a href="https://www.oreilly.com/library/view/command-line-rust/9781098109424/">Command-Line Rust</a></b>
                                     Rather than focusing on the language as a whole, this guide teaches Rust using a single small, complete, focused (CLI) program in each chapter. This book is lesser-known but seems to be <a href="https://www.reddit.com/r/rust/comments/1i4wv95/what_are_some_lesserknown_rust_books_worth_reading/m7yyr7e/">highly rated</a> by those who have read it.
                                  </li>
                                  <li>
                                    <b><a href="https://www.zero2prod.com/">Zero To Production In Rust</a></b>
                                    Aimed at developers with a background in backend web development. Starts with the basics of setting up a development environment, and takes the reader all the way through to creating a robust production-ready web service.
                                  </li>
                                  <li>
                                    <b><a href="https://www.rustinaction.com/">Rust In Action</a></b>
                                    This one has a slightly different focus, and is more suited to the more experienced developer with some grasp of CS theory, or those coming from a background in C or C++. It takes the reader through some relatively advanced projects including a CPU emulator, database, and an OS kernel.
                                  </li>
                                </ul>
                            "#,
                        }
                    }

                    Section {
                        section_key: "books".into(),
                        subsection_key: Cow::from("advanced-books"),
                        heading: "Advanced Books",
                        div {
                            dangerous_inner_html: r#"
                                <ul>
                                  <li>
                                    <b><a href="https://rust-unofficial.github.io/too-many-lists/">[Free] Learn Rust With Entirely Too Many Linked Lists</a></b>
                                    Note: Linked lists are not a beginner topic in Rust (even for those with extensive C experience). But if you're comfortable writing Rust on an everyday basis and want to really level up, then this is a fantastic hands-on way to do that.
                                  </li>
                                  <li>
                                    <b><a href="https://veykril.github.io/tlborm/">[Free] The Little Book of Rust Macros</a></b>
                                    A deep-dive into writing macros in Rust. The focus is on "macro_rules" macros. But there is also some content on procedural macros.
                                  </li>
                                  <li>
                                    <b><a href="https://nostarch.com/rust-rustaceans">Rust For Rustaceans</a></b>
                                    A more advanced book that dives into some of the more involved parts of Rust. Not suitable for a complete beginner (even if they have experience in other languages). Best read after you have already learnt some basic Rust through one or more of the other resources.
                                  </li>
                                  <li>
                                    <b><a href="https://marabos.nl/atomics/">Rust Atomics and Locks</a></b>
                                    A practical guide to understanding low-level concurrency. You’ll learn everything about atomics and memory ordering and how they're combined with basic operating system APIs.
                                  </li>
                                </ul>
                            "#,
                        }
                    }
                }

                Section { section_key: "other".into(), heading: "Other",
                    Section {
                        section_key: "other".into(),
                        subsection_key: Cow::from("by-doing"),
                        heading: "Learning by doing",
                        div { dangerous_inner_html: r#"
                                <ul>
                                  <li>
                                    <b><a href="https://github.com/rust-lang/rustlings">Rustlings</a></b>
                                    Small exercises to get you used to reading and writing Rust code, including reading and responding to compiler error messages.
                                  </li>
                                </ul>
                            "# }
                    }

                    Section {
                        section_key: "other".into(),
                        subsection_key: Cow::from("by-example"),
                        heading: "Learning by example",
                        div {
                            dangerous_inner_html: r#"
                                <ul>
                                  <li>
                                    <b><a href="https://cheats.rs/">Rust Language Cheat Sheet</a></b>
                                    A great simple quick-reference for all sorts of things from syntax to memory layouts of common types. Available in web and PDF form
                                  </li>
                                  <li>
                                    <b><a href="https://doc.rust-lang.org/rust-by-example/">Rust By Example</a></b>
                                    Learn rust features with small, targeted examples. Starting with "hello world" and moving up to more complex features like traits and generics.
                                  </li>
                                  <li>
                                    <b><a href="https://rust-lang-nursery.github.io/rust-cookbook/">The Rust Cookbook</a></b>
                                    A collection of simple examples that demonstrate good practices to accomplish common programming tasks, using the crates of the Rust ecosystem.
                                  </li>
                                </ul>
                            "#,
                        }
                    }

                    Section {
                        section_key: "other".into(),
                        subsection_key: Cow::from("videos"),
                        heading: "Videos",
                        div { dangerous_inner_html: r#"
                                <ul>
                                  <li>
                                    <b><a href="https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa">Crust of Rust</a></b>
                                    Intermediate level videos that explorer Rust by diving into real examples and working code, often by re-implementing functionality from the standard library. These episodes are not targeted at newcomers to Rust, but at those who have read The Book and have the theoretical knowledge, but want to see how these concepts translate into real use.
                                  </li>
                                </ul>
                            "# }
                    }
                }

                Section { section_key: "reference".into(), heading: "Reference",
                    Section {
                        section_key: "reference".into(),
                        subsection_key: Cow::from("official-docs"),
                        heading: "Official Documentation",
                        div {
                            dangerous_inner_html: r#"
                                <ul>
                                  <li>
                                    <b><a href="https://doc.rust-lang.org/stable/std/">The Standard Library Docs</a></b>
                                    Documentation for the built-in `std`, `core`, and `alloc` crates.
                                  </li>
                                  <li>
                                    <b><a href="https://doc.rust-lang.org/reference/">The Rust Reference</a></b>
                                    Defines details of the language semantics that you might expect to find in a specification if Rust had a specification.
                                  </li>
                                  <li>
                                    <b><a href="https://doc.rust-lang.org/reference/">The Rustinomicon</a></b>
                                    A high-level companion to the reference and a guide to correctly writing unsafe code in Rust.
                                  </li>
                                </ul>
                            "#,
                        }
                    }

                    Section {
                        section_key: "reference".into(),
                        subsection_key: Cow::from("unofficial-docs"),
                        heading: "Unofficial Documentation",
                        div { dangerous_inner_html: r#"
                                <ul>
                                  <li>
                                    <b><a href="https://spec.ferrocene.dev/">The Ferroscene Spec</a></b>
                                    An unnoficial but high quality specification for the Rust language developed by Ferrous Systems.
                                  </li>
                                </ul>
                            "# }
                    }
                }
            }
        }
    }
}
