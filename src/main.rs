mod handlers;
mod types;
mod utils;
mod config;

use std::net::{SocketAddr, IpAddr};
use config::create_app;

fn get_server_config() -> (IpAddr, u16) {
    let _ = dotenvy::dotenv();
    
    let ip_str = std::env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let ip: IpAddr = ip_str.parse().unwrap_or_else(|e| {
        eprintln!("Warning: Invalid SERVER_IP '{}': {}. Using default 127.0.0.1", ip_str, e);
        "127.0.0.1".parse().unwrap()
    });
    
    let port_str = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let port: u16 = port_str.parse().unwrap_or_else(|e| {
        eprintln!("Warning: Invalid SERVER_PORT '{}': {}. Using default 3000", port_str, e);
        3000
    });
    
    (ip, port)
}

#[tokio::main]
async fn main() {
    let (ip, port) = get_server_config();
    let addr = SocketAddr::new(ip, port);
    let app = create_app(addr);
    
    println!("Shoot for the Stars, Aim for the Moon");
    println!("📡 Listening on: http://{}", addr);
    println!("...Ready for it?");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}