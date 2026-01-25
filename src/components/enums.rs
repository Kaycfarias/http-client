use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    HTTPSelected(HTTPMethod),
    UrlChanged(String),
    HeaderKeyChanged(usize, String),
    HeaderValueChanged(usize, String),
    HeaderEnabledToggled(usize),
    AddHeader,
    RemoveHeader(usize),
    QueryParamKeyChanged(usize, String),
    QueryParamValueChanged(usize, String),
    QueryParamEnabledToggled(usize),
    AddQueryParam,
    RemoveQueryParam(usize),
    BodyChanged(String),
    BodyTypeChanged(BodyType),
    TimeoutChanged(String),
    Submit,
    #[allow(dead_code)]
    CancelRequest,
    RequestCompleted(Result<HttpResponse, String>),
    LoadFromHistory(usize),
    ClearHistory,
    TabChanged(RequestTab),
    ResponseTabChanged(ResponseTab),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HTTPMethod {
    #[default]
    GET,
    POST,
    PUT,
    #[allow(dead_code)]
    PATCH,
    DELETE,
}

impl HTTPMethod {
    pub fn as_reqwest(self) -> reqwest::Method {
        match self {
            HTTPMethod::GET => reqwest::Method::GET,
            HTTPMethod::POST => reqwest::Method::POST,
            HTTPMethod::PUT => reqwest::Method::PUT,
            HTTPMethod::PATCH => reqwest::Method::PATCH,
            HTTPMethod::DELETE => reqwest::Method::DELETE,
        }
    }

    #[allow(dead_code)]
    pub fn all() -> Vec<HTTPMethod> {
        vec![
            HTTPMethod::GET,
            HTTPMethod::POST,
            HTTPMethod::PUT,
            HTTPMethod::PATCH,
            HTTPMethod::DELETE,
        ]
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

impl KeyValue {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            enabled: true,
        }
    }

    pub fn empty() -> Self {
        Self {
            key: String::new(),
            value: String::new(),
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BodyType {
    #[default]
    None,
    Raw,
    Json,
}

impl std::fmt::Display for BodyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyType::None => write!(f, "None"),
            BodyType::Raw => write!(f, "Raw"),
            BodyType::Json => write!(f, "JSON"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HTTPMethod,
    pub url: String,
    pub headers: Vec<KeyValue>,
    pub query_params: Vec<KeyValue>,
    pub body: String,
    pub body_type: BodyType,
    pub timeout_ms: u64,
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            method: HTTPMethod::GET,
            url: String::new(),
            headers: vec![
                KeyValue::new("Content-Type".to_string(), "application/json".to_string()),
            ],
            query_params: Vec::new(),
            body: String::new(),
            body_type: BodyType::None,
            timeout_ms: 30000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub duration_ms: u128,
}

impl std::fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Status: {} {}\nTime: {}ms\n\nBody:\n{}",
            self.status, self.status_text, self.duration_ms, self.body
        )
    }
}

#[derive(Debug, Clone)]
pub struct HistoryItem {
    pub request: HttpRequest,
    pub response: HttpResponse,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestTab {
    QueryParams,
    Headers,
    Body,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseTab {
    Body,
    Headers,
}