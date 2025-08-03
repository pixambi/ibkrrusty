mod client;
mod endpoints;
mod models;

pub use client::{Client, ClientError};

pub use endpoints::session::SessionExt;
pub use models::session::{
    AuthStatusResponse, InitSessionRequest, InitSessionResponse, HmdsInitResponse,
    LogoutResponse, TickleResponse, SsoValidateResponse,
    ServerInfo, HmdsInfo, IServerInfo, Features
};

pub mod prelude {
    pub use crate::client::{Client, ClientError};
    pub use crate::endpoints::session::SessionExt;
}