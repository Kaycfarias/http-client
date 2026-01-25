use reqwest::Client;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use url::Url;
use tokio::runtime::Runtime;

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
    
    pub fn send_request(&self, request: HttpRequest, ) -> Result<HttpResponse, String> {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(async {

            // Validar e normalizar URL
            let validated_url = Self::validate_and_normalize_url(&request.url)?;
            
            // Construir URL com query params
            let full_url = Self::build_url_with_params(&validated_url, &request.query_params)?;
            
            // Construir headers
            let headers = Self::build_headers(&request.headers)?;

            // Iniciar timer
            let start = Instant::now();

            // Fazer a requisição
            let mut req_builder = self
                .client
                .request(request.method.as_reqwest(), &full_url)
                .timeout(Duration::from_millis(request.timeout_ms));

            // Adicionar headers
            for (key, value) in headers {
                req_builder = req_builder.header(key, value);
            }
            
            // Adicionar body se não for GET
            if request.method != super::enums::HTTPMethod::GET
            && !request.body.is_empty()
            && request.body_type != super::enums::BodyType::None
            {
                req_builder = req_builder.body(request.body.clone());
            }
            
            // Enviar requisição
            let response = req_builder
            .send()
            .await
            .map_err(|e| Self::format_error(e))?;
            
            // Calcular duração
            let duration = start.elapsed().as_millis();
            
            // Extrair informações da resposta
            let status = response.status().as_u16();
            let status_text = response.status().canonical_reason()
            .unwrap_or("Unknown")
            .to_string();

            let mut response_headers = HashMap::new();
            for (key, value) in response.headers().iter() {
                if let Ok(value_str) = value.to_str() {
                    response_headers.insert(key.to_string(), value_str.to_string());
                }
            }

            let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

            Ok(HttpResponse {
                status,
                status_text,
                body,
                headers: response_headers,
                duration_ms: duration,
            })
        })
    }

    fn validate_and_normalize_url(url: &str) -> Result<String, String> {
        let trimmed = url.trim();

        if trimmed.is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        // Auto-adicionar https:// se não tiver protocolo
        let normalized = if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
            format!("https://{}", trimmed)
        } else {
            trimmed.to_string()
        };

        // Validar URL
        Url::parse(&normalized)
            .map(|_| normalized)
            .map_err(|e| format!("Invalid URL: {}", e))
    }

    fn build_url_with_params(base_url: &str, params: &[KeyValue]) -> Result<String, String> {
        let mut url = Url::parse(base_url)
            .map_err(|e| format!("Failed to parse URL: {}", e))?;

        for param in params {
            if param.enabled && !param.key.is_empty() {
                url.query_pairs_mut()
                    .append_pair(&param.key, &param.value);
            }
        }

        Ok(url.to_string())
    }

    fn build_headers(headers: &[KeyValue]) -> Result<HashMap<String, String>, String> {
        let mut header_map = HashMap::new();

        for header in headers {
            if header.enabled && !header.key.is_empty() {
                header_map.insert(header.key.clone(), header.value.clone());
            }
        }

        Ok(header_map)
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