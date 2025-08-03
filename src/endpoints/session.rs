use crate::client::{Client, ClientError};
use crate::models::session::{
    AuthStatusResponse, InitSessionRequest, InitSessionResponse, HmdsInitResponse,
    LogoutResponse, TickleResponse, SsoValidateResponse
};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
pub trait SessionExt {
    async fn auth_status(&self) -> Result<AuthStatusResponse, ClientError>;

    async fn init_session(&self, compete: bool) -> Result<InitSessionResponse, ClientError>;

    async fn init_hmds(&self) -> Result<HmdsInitResponse, ClientError>;

    async fn logout(&self) -> Result<LogoutResponse, ClientError>;

    async fn tickle(&self) -> Result<TickleResponse, ClientError>;

    async fn validate_sso(&self) -> Result<SsoValidateResponse, ClientError>;
}

#[async_trait]
impl SessionExt for Client {
    async fn auth_status(&self) -> Result<AuthStatusResponse, ClientError> {
        let url = self.base_url.join("iserver/auth/status")?;

        let response = self
            .request(Method::POST, url)?
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await?;

        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;

        let auth_status = response.json::<AuthStatusResponse>().await?;
        Ok(auth_status)
    }

    async fn init_session(&self, compete: bool) -> Result<InitSessionResponse, ClientError> {
        let url = self.base_url.join("iserver/auth/ssodh/init")?;
        let body = InitSessionRequest::new(compete);

        let response = self
            .request(Method::POST, url)?
            .json(&body)
            .send()
            .await?;

        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;

        let init_response = response.json::<InitSessionResponse>().await?;
        Ok(init_response)
    }

    async fn init_hmds(&self) -> Result<HmdsInitResponse, ClientError> {
        let url = self.base_url.join("hmds/auth/init")?;

        let response = self
            .request(Method::POST, url)?
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await?;

        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;

        let hmds_response = response.json::<HmdsInitResponse>().await?;
        Ok(hmds_response)
    }

    async fn logout(&self) -> Result<LogoutResponse, ClientError> {
        let url = self.base_url.join("logout")?;

        let response = self
            .request(Method::POST, url)?
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await?;

        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;

        let logout_response = response.json::<LogoutResponse>().await?;
        Ok(logout_response)
    }

    async fn tickle(&self) -> Result<TickleResponse, ClientError> {
        let url = self.base_url.join("tickle")?;

        let response = self
            .request(Method::POST, url)?
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await?;

        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;

        let tickle_response = response.json::<TickleResponse>().await?;
        Ok(tickle_response)
    }
    
    async fn validate_sso(&self) -> Result<SsoValidateResponse, ClientError> {
        let url = self.base_url.join("sso/validate")?;

        let response = self
            .request(Method::GET, url)?
            .send()
            .await?;

        let response = response
            .error_for_status()
            .map_err(ClientError::RequestError)?;

        let sso_response = response.json::<SsoValidateResponse>().await?;
        Ok(sso_response)
    }
}