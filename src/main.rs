mod handlers;
mod types;
mod utils;
mod config;

use std::net::SocketAddr;
use config::create_app;

#[tokio::main]
async fn main() {
    let app = create_app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    println!("Shoot for the Stars, Aim for the Moon");
    println!("📡 Listening on: http://{}", addr);
    println!("...Ready for it?");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}