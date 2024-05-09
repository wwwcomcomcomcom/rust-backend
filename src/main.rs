use std::net::TcpListener;

use rust_backend::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("failed to bind random port");
    run(listener)?.await
}
