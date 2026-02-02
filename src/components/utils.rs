/// Utilitários para validação e formatação de URLs
pub mod url_validator {
    use url::Url;

    /// Valida e normaliza uma URL, adicionando https:// se necessário
    pub fn validate_and_normalize(input: &str) -> Result<String, String> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        let normalized = if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
            format!("https://{}", trimmed)
        } else {
            trimmed.to_string()
        };

        match Url::parse(&normalized) {
            Ok(_) => Ok(normalized),
            Err(e) => Err(format!("Invalid URL: {}", e)),
        }
    }

    /// Verifica se uma URL é válida
    #[allow(dead_code)]
    pub fn is_valid(url: &str) -> bool {
        validate_and_normalize(url).is_ok()
    }

    /// Extrai o domínio de uma URL
    #[allow(dead_code)]
    pub fn extract_domain(url: &str) -> Option<String> {
        Url::parse(url).ok().and_then(|u| u.host_str().map(String::from))
    }
}

/// Utilitários para formatação de JSON
pub mod json_formatter {
    use serde_json::Value;

    /// Formata JSON com pretty print
    pub fn format(json_str: &str) -> Result<String, String> {
        serde_json::from_str::<Value>(json_str)
            .map(|v| serde_json::to_string_pretty(&v).unwrap_or_else(|_| json_str.to_string()))
            .map_err(|e| format!("Invalid JSON: {}", e))
    }

    /// Verifica se uma string é JSON válido
    pub fn is_valid_json(json_str: &str) -> bool {
        serde_json::from_str::<Value>(json_str).is_ok()
    }

    /// Minifica JSON
    #[allow(dead_code)]
    pub fn minify(json_str: &str) -> Result<String, String> {
        serde_json::from_str::<Value>(json_str)
            .map(|v| serde_json::to_string(&v).unwrap_or_else(|_| json_str.to_string()))
            .map_err(|e| format!("Invalid JSON: {}", e))
    }
}

/// Utilitários para formatação de texto
pub mod text_formatter {
    /// Trunca texto se for muito longo
    #[allow(dead_code)]
    pub fn truncate(text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            text.to_string()
        } else {
            format!("{}...", &text[..max_length])
        }
    }

    /// Formata duração em milissegundos de forma legível
    pub fn format_duration(ms: u128) -> String {
        if ms < 1000 {
            format!("{}ms", ms)
        } else if ms < 60_000 {
            format!("{:.2}s", ms as f64 / 1000.0)
        } else {
            let seconds = ms / 1000;
            let minutes = seconds / 60;
            let remaining_seconds = seconds % 60;
            format!("{}m {}s", minutes, remaining_seconds)
        }
    }

    /// Formata tamanho de bytes de forma legível
    #[allow(dead_code)]
    pub fn format_bytes(bytes: usize) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;

        let bytes_f64 = bytes as f64;

        if bytes_f64 < KB {
            format!("{} B", bytes)
        } else if bytes_f64 < MB {
            format!("{:.2} KB", bytes_f64 / KB)
        } else if bytes_f64 < GB {
            format!("{:.2} MB", bytes_f64 / MB)
        } else {
            format!("{:.2} GB", bytes_f64 / GB)
        }
    }
}

/// Utilitários para exportação
pub mod export {
    use super::super::enums::{HttpRequest, KeyValue};

    /// Exporta uma requisição como comando curl
    #[allow(dead_code)]
    pub fn to_curl(request: &HttpRequest, full_url: &str) -> String {
        let mut curl = format!("curl -X {} '{}'", request.method, full_url);

        for header in &request.headers {
            if header.enabled && !header.key.is_empty() {
                curl.push_str(&format!(" \\\n  -H '{}: {}'", header.key, header.value));
            }
        }

        if !request.body.is_empty() && request.body_type != super::super::enums::BodyType::None {
            let escaped_body = request.body.replace('\'', "'\\''");
            curl.push_str(&format!(" \\\n  -d '{}'", escaped_body));
        }

        curl
    }

    /// Exporta headers como string formatada
    #[allow(dead_code)]
    pub fn headers_to_string(headers: &[KeyValue]) -> String {
        headers
            .iter()
            .filter(|h| h.enabled && !h.key.is_empty())
            .map(|h| format!("{}: {}", h.key, h.value))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_validation() {
        assert!(url_validator::is_valid("https://example.com"));
        assert!(!url_validator::is_valid(""));
        assert!(!url_validator::is_valid("not a url"));
    }

    #[test]
    fn test_json_formatter() {
        let json = r#"{"name":"John","age":30}"#;
        assert!(json_formatter::is_valid_json(json));
        assert!(json_formatter::format(json).is_ok());
    }

    #[test]
    fn test_duration_formatter() {
        assert_eq!(text_formatter::format_duration(500), "500ms");
        assert_eq!(text_formatter::format_duration(1500), "1.50s");
    }
}
