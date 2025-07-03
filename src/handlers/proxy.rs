use axum::{
    extract::{Query, Request, State},
    response::Response,
};
use std::collections::HashMap;
use futures_util::TryStreamExt;

use crate::handlers::manifest::rewrite_manifest_urls;
use crate::types::{StreamType, ProxyParams};
use crate::utils::{
    headers::{build_request_headers, build_response_headers},
    client::create_optimized_client,
};
use crate::config::AppState;

pub async fn proxy_handler(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
    request: Request<axum::body::Body>,
) -> Response {
    let Some(url) = params.get("url") else {
        return Response::builder()
            .status(400)
            .body("Missing ?url= parameter".into())
            .unwrap();
    };

    let client_range = request.headers().get("range")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let mut proxy_params = ProxyParams::from_query_params(&params);
    
    if let Some(range) = client_range {
        proxy_params.range = Some(range);
    }
    
    let client = create_optimized_client();
    
    let request = build_request_headers(client.get(url), &proxy_params, url);

    let Ok(resp) = request.send().await else {
        return Response::builder().status(502).body("Upstream error".into()).unwrap();
    };

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .cloned()
        .unwrap_or_else(|| "application/vnd.apple.mpegurl".parse().unwrap());

    let response_headers = build_response_headers(&resp);

    let stream_type = StreamType::from_params(&proxy_params.stream_type, url, &content_type);

    match stream_type {
        StreamType::Manifest => handle_manifest_response(resp, url, status, content_type, state.server_addr).await,
        StreamType::Mp4 => handle_streaming_response(resp, status, response_headers, "video/mp4").await,
        StreamType::Other => handle_streaming_response(resp, status, response_headers, content_type.to_str().unwrap_or("application/octet-stream")).await,
    }
}


async fn handle_manifest_response(
    resp: reqwest::Response,
    url: &str,
    status: reqwest::StatusCode,
    content_type: axum::http::HeaderValue,
    server_addr: std::net::SocketAddr,
) -> Response {
  
    let body_bytes = match resp.bytes().await {
        Ok(bytes) => bytes,
        Err(_) => return Response::builder().status(502).body("Failed to read response".into()).unwrap(),
    };
    
    let manifest_content = String::from_utf8_lossy(&body_bytes);
    let base_url = if let Ok(parsed_url) = url::Url::parse(url) {
        parsed_url.join("./").unwrap().to_string()
    } else {
        url.to_string()
    };
    
  
    let rewritten_manifest = rewrite_manifest_urls(&manifest_content, &base_url, server_addr);
    
    Response::builder()
        .status(status)
        .header("content-type", content_type)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "*")
        .header("Connection", "close")
        .body(rewritten_manifest.into())
        .unwrap()
}

async fn handle_streaming_response(
    resp: reqwest::Response,
    status: reqwest::StatusCode,
    response_headers: HashMap<String, axum::http::HeaderValue>,
    content_type: &str,
) -> Response {
   
    let stream = resp
        .bytes_stream()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));

    let body = axum::body::Body::from_stream(stream);

    let mut response_builder = Response::builder()
        .status(status) 
        .header("content-type", content_type)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "*")
        .header("Connection", "close")
        .header("Cache-Control", "no-cache"); 
    
    if content_type == "video/mp4" || content_type.starts_with("video/") {
        response_builder = response_builder.header("Accept-Ranges", "bytes");
    }
    
    for (key, value) in response_headers {
        response_builder = response_builder.header(key, value);
    }

    response_builder.body(body).unwrap()
}
