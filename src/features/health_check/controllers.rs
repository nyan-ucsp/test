use actix_web::{get, HttpResponse, Responder};

use crate::common::models::response_message::ResponseMessage;

/// Check Health
///
/// Checking Service Health
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Request successfully", body = ResponseMessage),
    ),
    tag = "HealthCheck",
)]
#[get("/health")]
pub async fn get_health() -> impl Responder {
    HttpResponse::Ok().json(ResponseMessage {
        message: String::from("Service is running"),
    })
}
