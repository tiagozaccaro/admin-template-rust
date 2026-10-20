use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthUrlResponse {
    pub auth_url: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct MeResponse {
    pub user: Option<contracts::User>,
    pub session: Option<serde_json::Value>,
    pub active_tenant: Option<contracts::Tenant>,
}

pub fn router() -> Router {
    Router::new()
        .route("/login/google/start", post(login_google_start))
        .route("/callback/google", get(callback_google))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

async fn login_google_start() -> Json<AuthUrlResponse> {
    Json(AuthUrlResponse {
        auth_url: "https://example.com/auth".to_string(),
    })
}

async fn callback_google() -> Json<LogoutResponse> {
    Json(LogoutResponse { success: true })
}

async fn logout() -> Json<LogoutResponse> {
    Json(LogoutResponse { success: true })
}

async fn me() -> Json<MeResponse> {
    Json(MeResponse {
        user: None,
        session: None,
        active_tenant: None,
    })
}
