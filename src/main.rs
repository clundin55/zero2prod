use std::net::TcpListener;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:10000")?;
    zero2prod::run(listener).await
}
