use std::net::SocketAddr;

pub fn rewrite_manifest_urls(manifest: &str, base_url: &str, server_addr: SocketAddr) -> String {
    let mut result = String::new();
    let base_proxy_url = format!("http://{}", server_addr);
    
    for line in manifest.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            result.push_str(line);
            result.push('\n');
        } else {
            let full_url = resolve_url(line, base_url);
            
           let proxy_url = format!("{}/proxy?url={}", base_proxy_url, urlencoding::encode(&full_url));

            result.push_str(&proxy_url);
            result.push('\n');
        }
    }
    
    result
}

fn resolve_url(line: &str, base_url: &str) -> String {
    if line.starts_with("http://") || line.starts_with("https://") {
        line.to_string()
    } else {
        if let Ok(base) = url::Url::parse(base_url) {
            if let Ok(joined) = base.join(line) {
                joined.to_string()
            } else {
                line.to_string()
            }
        } else {
            line.to_string()
        }
    }
}
