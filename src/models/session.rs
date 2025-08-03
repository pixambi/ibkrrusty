use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatusResponse {
    pub authenticated: bool,
    pub competing: bool,
    pub connected: bool,
    pub message: String,
    #[serde(rename = "MAC")]
    pub mac: String,
    pub server_info: Option<ServerInfo>,
    pub hardware_info: Option<String>,
    pub fail: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub server_name: String,
    pub server_version: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct InitSessionRequest {
    pub publish: bool,
    pub compete: bool,
}

impl InitSessionRequest {
    pub fn new(compete: bool) -> Self {
        Self {
            publish: true, // Always true as per documentation
            compete,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitSessionResponse {
    pub authenticated: bool,
    pub competing: bool,
    pub connected: bool,
    pub message: String,
    #[serde(rename = "MAC")]
    pub mac: String,
    pub server_info: Option<ServerInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HmdsInitResponse {
    pub authenticated: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogoutResponse {
    pub status: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TickleResponse {
    pub session: String,
    pub sso_expires: i64,
    pub collission: bool,
    pub user_id: i64,
    pub hmds: Option<HmdsInfo>,
    pub iserver: Option<IServerInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HmdsInfo {
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IServerInfo {
    pub auth_status: AuthStatusResponse,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SsoValidateResponse {
    pub user_id: i64,
    pub user_name: String,
    pub result: bool,
    pub auth_time: i64,
    pub sf_enabled: bool,
    pub is_free_trial: bool,
    pub credential: String,
    pub ip: String,
    pub expires: i64,
    pub qualified_for_mobile_auth: Option<bool>,
    pub landing_app: String,
    pub is_master: bool,
    pub last_accessed: i64,
    pub login_type: i32,
    pub paper_user_name: Option<String>,
    pub features: Option<Features>,
    pub region: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Features {
    pub env: String,
    pub wlms: bool,
    pub realtime: bool,
    pub bond: bool,
    #[serde(rename = "optionChains")]
    pub option_chains: bool,
    pub calendar: bool,
    #[serde(rename = "newMf")]
    pub new_mf: bool,
}