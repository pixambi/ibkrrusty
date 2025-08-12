use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client as HttpClient, Method, RequestBuilder};
use url::{ParseError, Url};

#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) http: HttpClient,
    pub(crate) base_url: url::Url,
}

#[derive(Debug)]
pub enum ClientError {
    ParseError(ParseError),
    RequestError(reqwest::Error),
    EnvError(String),
}

impl From<ParseError> for ClientError {
    fn from(err: ParseError) -> Self {
        ClientError::ParseError(err)
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        ClientError::RequestError(err)
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::ParseError(e) => write!(f, "URL parse error: {}", e),
            ClientError::RequestError(e) => write!(f, "Request error: {}", e),
            ClientError::EnvError(e) => write!(f, "Environment error: {}", e),
        }
    }
}

impl std::error::Error for ClientError {}

impl Client {
    pub fn new() -> Result<Self, ClientError> {
        let port = Self::get_port_from_env();
        let base_url = format!("https://localhost:{}/v1/api/", port);
        Self::with_base_url(&base_url)
    }

    pub fn with_base_url(base_url: &str) -> Result<Self, ClientError> {
        Ok(Self {
            http: HttpClient::builder()
                .danger_accept_invalid_certs(true)
                .cookie_store(true)
                .build()
                .map_err(ClientError::RequestError)?,
            base_url: url::Url::parse(base_url)?,
        })
    }

    pub fn with_port(port: u16) -> Result<Self, ClientError> {
        let base_url = format!("https://localhost:{}/v1/api/", port);
        Self::with_base_url(&base_url)
    }

    pub fn with_client(http_client: HttpClient, base_url: Option<&str>) -> Result<Self, ClientError> {
        let url = base_url.unwrap_or("https://localhost:5000/v1/api/");
        Ok(Self {
            http: http_client,
            base_url: url::Url::parse(url)?,
        })
    }

    fn get_port_from_env() -> u16 {
        std::env::var("PORT")
            .ok()
            .and_then(|port_str| port_str.parse().ok())
            .unwrap_or(5000)
    }

    pub(crate) fn request(
        &self,
        method: Method,
        url: url::Url,
    ) -> Result<RequestBuilder, ClientError> {
        let headers = self.default_headers()?;
        Ok(self.http.request(method, url).headers(headers))
    }

    pub(crate) async fn handle_no_content_response(
        &self,
        response: reqwest::Response,
    ) -> Result<(), ClientError> {
        match response.status() {
            reqwest::StatusCode::NO_CONTENT => Ok(()),
            _ => {
                response
                    .error_for_status()
                    .map(|_| ())
                    .map_err(ClientError::RequestError)
            }
        }
    }

    fn default_headers(&self) -> Result<HeaderMap, ClientError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "User-Agent",
            HeaderValue::from_static("ibkrrusty/0.1.0")
        );
        Ok(headers)
    }
}