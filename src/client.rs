use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
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
    pub fn new(token: &str) -> Result<Self, ClientError> {
        Ok(Self {
            http: HttpClient::new(),
            base_url: url::Url::parse("https://localhost:5000/v1/api/")?,
        })
    }

    //Request builder with authentication
    pub(crate) fn request(
        &self,
        method: Method,
        url: url::Url,
    ) -> Result<RequestBuilder, ClientError> {
        let headers = self.auth_headers()?;
        Ok(self.http.request(method, url).headers(headers))
    }

    /// Helper method to handle responses that should return 204 No Content
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

    fn auth_headers(&self) -> Result<HeaderMap, ClientError> {
        let headers = HeaderMap::new();
        Ok(headers)
    }
}
