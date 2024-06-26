//! main.rs

use hatchet::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000").expect("Failed to bind port 8000.");
    run(listener)?.await
}
