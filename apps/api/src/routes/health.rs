use axum::response::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    pub status: &'static str,
}

#[derive(Serialize)]
pub struct Ready {
    pub status: &'static str,
    pub db: &'static str,
}

pub async fn healthz() -> Json<Health> {
    Json(Health { status: "ok" })
}

pub async fn readyz() -> Json<Ready> {
    Json(Ready {
        status: "ok",
        db: "ok",
    })
}
