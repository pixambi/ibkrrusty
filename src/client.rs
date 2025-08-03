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

impl Client {

    pub fn new(base_url: Option<&str>) -> Result<Self, ClientError> {
        let url = base_url.unwrap_or("https://localhost:5000/v1/api/");

        Ok(Self {
            http: HttpClient::builder()
                .danger_accept_invalid_certs(true) // IBKR gateway uses self-signed certificates
                .build()
                .map_err(ClientError::RequestError)?,
            base_url: url::Url::parse(url)?,
        })
    }

    pub fn with_client(http_client: HttpClient, base_url: Option<&str>) -> Result<Self, ClientError> {
        let url = base_url.unwrap_or("https://localhost:5000/v1/api/");

        Ok(Self {
            http: http_client,
            base_url: url::Url::parse(url)?,
        })
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
                    .map(|_| ()) // Convert success response to ()
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