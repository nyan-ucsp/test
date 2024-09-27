use actix_multipart::Multipart;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use crate::common::database::DbPool;
use crate::common::enums::Role::Admin;
use crate::common::models::response_message::ResponseMessage;
use crate::common::utils::{delete_directory_if_exists, parse_payload_data};
use crate::features::episode::models::CreateEpisodeRequest;
use crate::features::check_role;
use crate::features::episode::services::Service;

/// Create Episode
///
/// Create a new episode
#[utoipa::path(
    post,
    path = "/episode",
    request_body(
        content = CreateEpisodeRequest,
        description = "Create Episode",
        content_type = "multipart/form-data",
    ),
    responses(
        (status = 201, description = "Created successfully", body = EpisodeResponse),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Episode",
)]
#[post("/episode")]
pub async fn create_episode(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    payload: Multipart,
) -> impl Responder {
    if check_role(http_request) == Admin {
        match parse_payload_data(payload).await {
            Ok((payload_data, tmp_path)) => {
                let check_required_fields = CreateEpisodeRequest::check_required_data(payload_data.clone()).await;
                if check_required_fields {
                    let req_data = CreateEpisodeRequest::from_payload_data(payload_data).await;
                    match Service::create_episode(&pool, req_data).await {
                        Ok(new_episode) => {
                            delete_directory_if_exists(&tmp_path);
                            HttpResponse::Created().json(new_episode)
                        }
                        Err(e) => {
                            println!("Failed to create album: {}", e);
                            delete_directory_if_exists(&tmp_path);
                            HttpResponse::InternalServerError().json(ResponseMessage {
                                message: String::from("Internal Server Error"),
                            })
                        }
                    }
                } else {
                    HttpResponse::BadRequest().json(ResponseMessage {
                        message: String::from("Invalid request model"),
                    })
                }
            }
            Err(e) => HttpResponse::BadRequest().json(ResponseMessage {
                message: String::from(e),
            }),
        }
    } else {
        HttpResponse::Unauthorized().json(ResponseMessage {
            message: String::from("Unauthorized"),
        })
    }
}