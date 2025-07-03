use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProxyParams {
    pub stream_type: Option<String>,
    pub referer: Option<String>,
    pub authorization: Option<String>,
    pub range: Option<String>,
    pub custom_headers: HashMap<String, String>,
}

impl ProxyParams {
    pub fn from_query_params(params: &HashMap<String, String>) -> Self {
        let mut custom_headers = HashMap::new();
        
        for (key, value) in params {
            if key.starts_with("header_") {
                let header_name = &key[7..]; 
                custom_headers.insert(header_name.to_string(), value.clone());
            }
        }

        Self {
            stream_type: params.get("type").cloned(),
            referer: params.get("referer").cloned(),
            authorization: params.get("authorization").cloned(),
            range: params.get("range").cloned(),
            custom_headers,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StreamType {
    Manifest,
    Mp4,
    Other,
}

impl StreamType {
    pub fn from_params(
        stream_type_param: &Option<String>,
        url: &str,
        content_type: &axum::http::HeaderValue,
    ) -> Self {
        let stream_type = stream_type_param.as_ref().map(|s| s.as_str()).unwrap_or("auto");
        
        match stream_type {
            "hls" => StreamType::Manifest,
            "mp4" => StreamType::Mp4,
            "auto" => {
                if url.ends_with(".m3u8") || content_type.to_str().unwrap_or("").contains("mpegurl") {
                    StreamType::Manifest
                } else if url.ends_with(".mp4") || content_type.to_str().unwrap_or("").contains("video/mp4") {
                    StreamType::Mp4
                } else {
                    StreamType::Other
                }
            }
            _ => {
                if url.ends_with(".m3u8") || content_type.to_str().unwrap_or("").contains("mpegurl") {
                    StreamType::Manifest
                } else if url.ends_with(".mp4") || content_type.to_str().unwrap_or("").contains("video/mp4") {
                    StreamType::Mp4
                } else {
                    StreamType::Other
                }
            }
        }
    }
}
