use tera::{Tera, Context};
use once_cell::sync::Lazy;

pub(crate) static TERA: Lazy<Tera> = Lazy::new(|| {
    let mut tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    // let mut tera = Tera::default();
    // tera.add_raw_template("list.html", include_str!("list.html")).unwrap();
    tera.autoescape_on(vec![".html", ".sql"]);
    // tera.register_filter("do_nothing", do_nothing_filter);
    tera
});