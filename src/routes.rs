pub mod users;
pub mod crates;
pub mod getting_started;

// basic handler that responds with a static string
pub async fn index() -> &'static str {
    "Hello, World!"
}