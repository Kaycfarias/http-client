#[derive(Debug, Clone)]
pub enum Message {
    HTTPSelected(HTTPMethod),
    Edit(String),
    RequestCompleted(Result<String, String>),
    Submit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HTTPMethod {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
}

impl HTTPMethod {
    pub fn as_reqwest(self) -> reqwest::Method {
        match self {
            HTTPMethod::GET => reqwest::Method::GET,
            HTTPMethod::POST => reqwest::Method::POST,
            HTTPMethod::PUT => reqwest::Method::PUT,
            HTTPMethod::DELETE => reqwest::Method::DELETE,
        }
    }
}