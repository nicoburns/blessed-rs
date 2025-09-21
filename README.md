# [Blessed.rs](https://blessed.rs)

A community guide to the Rust ecosystem

## Testing locally

- Run `cargo run` then open http://localhost:3333 in your browser
- The port can be overridden using the `PORT` environment variable
- If you want to automatically recompile and re-run on save you can install [cargo-watch](https://crates.io/crates/cargo-watch) (`cargo install cargo-watch`) then and use `cargo watch -x run` instead of `cargo run`.