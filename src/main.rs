use rust_backend::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let _ = run().await?; // Explicitly await the run() function call
    Ok(())
}
