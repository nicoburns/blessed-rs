// pub mod dioxus_elements {
//     pub use dioxus::prelude::dioxus_elements::*;
//     pub mod html {
//         pub const TAG_NAME: &'static str = "html";
//         pub const NAME_SPACE: Option<&'static str> = None;
//         pub use dioxus::prelude::global_attributes::*;
//     }
// }
#![allow(unused_imports)]

mod page;
pub use page::{MainContent, Page};
mod section;
pub use section::{Section, SectionLevel};
mod toc;
pub use toc::{TableOfContents, TocSection, TocSubSection};