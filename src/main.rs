use std::net::TcpListener;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:0")?;
    zero2prod::startup::run(listener).await
}
