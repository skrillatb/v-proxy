use reqwest::Client;
use std::time::Duration;

pub fn create_optimized_client() -> Client {
    Client::builder()
        // NE PAS POOL LA CONNEXION IMPORTANT 
        .pool_max_idle_per_host(0) 

        .pool_idle_timeout(Duration::from_secs(0))
        .timeout(Duration::from_secs(30)) 
        .tcp_keepalive(None) 
        .http2_keep_alive_interval(None) 
        .build()
        .unwrap_or_else(|_| Client::new())
}
