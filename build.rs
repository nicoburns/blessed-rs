use std::fs;
use std::path::PathBuf;

fn get_input_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::with_capacity(50);
    path.push(".");
    path.push("styles");
    path.push(filename);
    path.set_extension("scss");
    path
}

fn get_output_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::with_capacity(50);
    path.push(".");
    path.push("static");
    path.push(filename);
    path.set_extension("css");
    path
}

fn main() {
    println!("cargo:rerun-if-changed=styles");

    let entrypoints = ["index"];
    for filename in &entrypoints {
        let input_path = get_input_path(filename);
        let css = grass::from_path(&input_path, &grass::Options::default()).unwrap();

        let output_path = get_output_path(filename);
        fs::write(output_path, css).unwrap();
    }
}
