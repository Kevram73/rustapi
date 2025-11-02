use rocket::serde::json::Json;
use crate::api::dto::ApiResponse;

// Handler de santé générique
#[rocket::get("/health")]
pub fn health_check() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })))
}

#[rocket::get("/")]
pub fn root() -> Json<ApiResponse<serde_json::Value>> {
    health_check()
}

