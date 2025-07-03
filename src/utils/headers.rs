use reqwest::RequestBuilder;
use std::collections::HashMap;
use axum::http::HeaderValue;

use crate::types::ProxyParams;

pub fn build_request_headers(
    mut request: RequestBuilder,
    params: &ProxyParams,
    url: &str,
) -> RequestBuilder {
    request = request
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header("Accept", "*/*")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Connection", "close") // Don't use keep-alive for memory optimization
        .header("Upgrade-Insecure-Requests", "1")
        .header("Sec-Fetch-Dest", "video")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "cross-site");
    
    if let Some(referer) = &params.referer {
        request = request.header("Referer", referer);
    } else {
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(host) = parsed_url.host_str() {
                let referer = format!("https://{}/", host);
                request = request.header("Referer", referer);
            }
        }
    }
    
    if let Some(auth) = &params.authorization {
        request = request.header("Authorization", auth);
    }
    
    for (key, value) in &params.custom_headers {
        request = request.header(key, value);
    }
    
    if let Some(range) = &params.range {
        if range.starts_with("bytes=") {
            request = request.header("Range", range);
        } else {
            request = request.header("Range", format!("bytes={}", range));
        }
    }

    request
}

pub fn build_response_headers(resp: &reqwest::Response) -> HashMap<String, HeaderValue> {
    let mut headers = HashMap::new();
    
    if let Some(content_length) = resp.headers().get("content-length") {
        headers.insert("content-length".to_string(), content_length.clone());
    }
    if let Some(content_range) = resp.headers().get("content-range") {
        headers.insert("content-range".to_string(), content_range.clone());
    }
    if let Some(last_modified) = resp.headers().get("last-modified") {
        headers.insert("last-modified".to_string(), last_modified.clone());
    }
    if let Some(etag) = resp.headers().get("etag") {
        headers.insert("etag".to_string(), etag.clone());
    }
    
    headers
}
