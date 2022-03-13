pub mod users;
pub mod crates;

// basic handler that responds with a static string
pub async fn index() -> &'static str {
    "Hello, World!"
}