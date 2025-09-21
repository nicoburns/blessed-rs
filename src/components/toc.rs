use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TocSubSection {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TocSection {
    pub name: String,
    pub slug: String,
    pub subsections: Vec<TocSubSection>,
}

#[component]
pub fn TableOfContents(sections: Vec<TocSection>) -> Element {
    rsx! {
        div { class: "toc",
            h3 { "Sections" }
            ul {
                for section in sections.into_iter() {
                    TocSectionRender { section }
                }
            }
            script { dangerous_inner_html: TOC_SCRIPT }
        }
    }
}

#[component]
fn TocSectionRender(section: TocSection) -> Element {
    rsx! {
        li { "data-toc-link": "section-{ section.slug }",
            a { href: "#section-{ section.slug }", "{ section.name }" }
            if !section.subsections.is_empty() {
                ul { class: "subtoc",
                    for subsection in section.subsections.iter() {
                        li { "data-toc-link": "section-{ section.slug }-subsection-{ subsection.slug }",
                            a { href: "#section-{ section.slug }-subsection-{ subsection.slug }",
                                "{ subsection.name }"
                            }
                        }
                    }
                }
            }
        }
    }
}

const TOC_SCRIPT: &str = r#"
  window.addEventListener('DOMContentLoaded', () => {

    const observer = new IntersectionObserver(entries => {
      entries.forEach(entry => {
        const id = entry.target.getAttribute('id');
        if (entry.intersectionRatio > 0) {
          document.querySelector(`li[data-toc-link="${id}"]`).classList.add('active');
        } else {
          document.querySelector(`li[data-toc-link="${id}"]`).classList.remove('active');
        }
      });
    });

    // Track all sections that have an `id` applied
    document.querySelectorAll('section[data-toc-section]').forEach((section) => {
      observer.observe(section);
    });

  });
"#;
