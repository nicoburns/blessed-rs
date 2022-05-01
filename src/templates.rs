use tera::Tera;
use once_cell::sync::Lazy;
use fxhash::hash32;
use std::fs::read_to_string;
use std::env::current_dir;
use serde_json::{ Value,  Number };

pub(crate) static TERA: Lazy<Tera> = Lazy::new(|| {
    let mut tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    // Register function to get hash of CSS file. Hash doesn't need to be secure as it is
    // purely to prevent the old version of the file being cached when the file it updated
    let cwd = current_dir().unwrap();
    let index_css_path = { let mut path = cwd.clone(); path.push("static/index.css"); path };
    let index_css_contents = read_to_string(index_css_path).unwrap();
    let index_css_hash = hash32(&index_css_contents);
    let index_css_hash_json_value = Value::Number(Number::from_f64(f64::from(index_css_hash)).unwrap());
    tera.register_function("get_index_css_hash", move |_: &_| Ok(index_css_hash_json_value.clone()));

    // let mut tera = Tera::default();
    // tera.add_raw_template("list.html", include_str!("list.html")).unwrap();
    tera.autoescape_on(vec![".html", ".sql"]);
    // tera.register_filter("do_nothing", do_nothing_filter);
    tera
});