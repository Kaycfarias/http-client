use reqwest::Client;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use url::Url;

use super::enums::{HttpRequest, HttpResponse, KeyValue};

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub fn send_request(&self, request: HttpRequest) -> Result<HttpResponse, String> {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            let validated_url = Self::validate_and_normalize_url(&request.url)?;
            let full_url = Self::build_url_with_params(&validated_url, &request.query_params)?;
            let headers = Self::build_headers(&request.headers)?;

            let start = Instant::now();
            let req_builder = self.build_request_with_body(&request, &full_url, headers);
            let response = req_builder.send().await.map_err(Self::format_error)?;
            let duration_ms = start.elapsed().as_millis();

            Self::process_response(response, duration_ms).await
        })
    }

    fn build_request_with_body(
        &self,
        request: &HttpRequest,
        url: &str,
        headers: HashMap<String, String>,
    ) -> reqwest::RequestBuilder {
        let mut builder = self
            .client
            .request(request.method.as_reqwest(), url)
            .timeout(Duration::from_millis(request.timeout_ms));

        for (key, value) in headers {
            builder = builder.header(key, value);
        }

        if Self::should_include_body(request) {
            builder = builder.body(request.body.clone());
        }

        builder
    }

    fn should_include_body(request: &HttpRequest) -> bool {
        request.method.allows_body()
            && !request.body.is_empty()
            && request.body_type != super::enums::BodyType::None
    }

    async fn process_response(
        response: reqwest::Response,
        duration_ms: u128,
    ) -> Result<HttpResponse, String> {
        let status = response.status().as_u16();
        let status_text = response
            .status()
            .canonical_reason()
            .unwrap_or("Unknown")
            .to_string();

        let response_headers = Self::extract_headers(response.headers());

        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        Ok(HttpResponse {
            status,
            status_text,
            body,
            headers: response_headers,
            duration_ms,
        })
    }

    fn extract_headers(headers: &reqwest::header::HeaderMap) -> HashMap<String, String> {
        headers
            .iter()
            .filter_map(|(key, value)| {
                value
                    .to_str()
                    .ok()
                    .map(|v| (key.to_string(), v.to_string()))
            })
            .collect()
    }

    fn validate_and_normalize_url(url: &str) -> Result<String, String> {
        let trimmed = url.trim();

        if trimmed.is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        let normalized = Self::add_protocol_if_missing(trimmed);
        Self::validate_url(&normalized)?;

        Ok(normalized)
    }

    fn add_protocol_if_missing(url: &str) -> String {
        if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            format!("https://{}", url)
        }
    }

    fn validate_url(url: &str) -> Result<(), String> {
        Url::parse(url)
            .map(|_| ())
            .map_err(|e| format!("Invalid URL: {}", e))
    }

    fn build_url_with_params(base_url: &str, params: &[KeyValue]) -> Result<String, String> {
        let mut url = Url::parse(base_url).map_err(|e| format!("Failed to parse URL: {}", e))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            for param in params.iter().filter(|p| p.enabled && !p.key.is_empty()) {
                query_pairs.append_pair(&param.key, &param.value);
            }
        }

        Ok(url.to_string())
    }

    fn build_headers(headers: &[KeyValue]) -> Result<HashMap<String, String>, String> {
        Ok(headers
            .iter()
            .filter(|h| h.enabled && !h.key.is_empty())
            .map(|h| (h.key.clone(), h.value.clone()))
            .collect())
    }

    fn format_error(error: reqwest::Error) -> String {
        if error.is_timeout() {
            "Request timeout - the server took too long to respond".to_string()
        } else if error.is_connect() {
            format!("Connection failed: {}", error)
        } else if error.is_request() {
            format!("Request error: {}", error)
        } else {
            format!("HTTP error: {}", error)
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
