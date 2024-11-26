<<<<<<< HEAD
mod groq_api;
use groq_api::groq_request;
#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    groq_request().await?;
    Ok(())
=======
fn main() {
    println!("Hello, world!");
>>>>>>> 6be4597 (First commit)
}
