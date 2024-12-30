# [Blessed.rs](https://blessed.rs)

A community guide to the Rust ecosystem

## Testing locally

```
cargo build --release
docker build . -t blessed-rs
docker run -p 3333:3333 blessed-rs
< check localhost:3333 in browser>
```