use crate::common::database::DbPool;
use crate::common::enums::Role::Admin;
use crate::common::models::response_message::ResponseMessage;
use crate::common::utils::{delete_directory_if_exists, parse_payload_data};
use crate::features::check_role;
use crate::features::content::models;
use crate::features::content::services::Service;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

/// Add Contents
///
/// Add contents to episode
#[utoipa::path(
    post,
    path = "/content",
    request_body(
        content = AddEpisodeContentsRequest,
        description = "Add Episode Contents Request",
        content_type = "multipart/form-data",
    ),
    responses(
        (status = 201, description = "Add successfully", body = ContentResponse),
        (status = 400, description = "Update failed", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Content",
)]
#[post("/content")]
pub async fn add_contents(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    payload: Multipart,
) -> impl Responder {
    if check_role(http_request) == Admin {
        match parse_payload_data(payload).await {
            Ok((payload_data, tmp_path)) => {
                match models::AddEpisodeContentsRequest::from_payload_data(payload_data).await {
                    Ok(req_data) => match Service::add_episode_contents(&pool, req_data).await {
                        Ok(response) => {
                            delete_directory_if_exists(&tmp_path);
                            HttpResponse::Created().json(response)
                        }
                        Err(e) => {
                            delete_directory_if_exists(&tmp_path);
                            HttpResponse::BadRequest().json(ResponseMessage {
                                message: String::from(e),
                            })
                        }
                    },
                    Err(e) => HttpResponse::BadRequest().json(ResponseMessage {
                        message: String::from(e),
                    }),
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

/// Get Contents
///
/// Get contents by episode id
#[utoipa::path(
    get,
    path = "/contents/{episode_uuid}",
    responses(
        (status = 200, description = "Add successfully", body = ContentResponse),
        (status = 400, description = "Update failed", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Content",
)]
#[get("/contents/{episode_uuid}")]
pub async fn get_contents(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let episode_uuid = path.into_inner();
    if check_role(http_request) == Admin {
        match Service::get_contents_by_episode_uuid(&pool, episode_uuid).await {
            Ok(response) => HttpResponse::Ok().json(response),
            Err(_) => HttpResponse::BadRequest().json(ResponseMessage {
                message: String::from("Failed to get contents data"),
            }),
        }
    } else {
        HttpResponse::Unauthorized().json(ResponseMessage {
            message: String::from("Unauthorized"),
        })
    }
}

/// Update Content
///
/// Update content
#[utoipa::path(
    post,
    path = "/contents/{content_uuid}",
    request_body(
        content = UpdateContentRequest,
        description = "Update Content Request",
        content_type = "multipart/form-data",
    ),
    responses(
        (status = 200, description = "Add successfully", body = ContentResponse),
        (status = 400, description = "Update failed", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Content",
)]
#[put("/contents/{content_uuid}")]
pub async fn update_content(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    path: web::Path<String>,
    payload: Multipart,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let content_uuid = path.into_inner();
        match parse_payload_data(payload).await {
            Ok((payload_data, tmp_path)) => {
                match models::UpdateContentRequest::from_payload_data(payload_data).await {
                    Ok(req_data) => match Service::update_content(&pool, content_uuid, req_data).await {
                        Ok(response) => {
                            delete_directory_if_exists(&tmp_path);
                            HttpResponse::Ok().json(response)
                        }
                        Err(e) => {
                            delete_directory_if_exists(&tmp_path);
                            HttpResponse::BadRequest().json(ResponseMessage {
                                message: String::from("Failed to update content"),
                            })
                        }
                    },
                    Err(e) => HttpResponse::BadRequest().json(ResponseMessage {
                        message: String::from(e),
                    }),
                }
            }
            Err(e) => HttpResponse::BadRequest().json(ResponseMessage {
                message: String::from("Failed to parse request data"),
            }),
        }
    } else {
        HttpResponse::Unauthorized().json(ResponseMessage {
            message: String::from("Unauthorized"),
        })
    }
}


/// Delete Content
///
/// Delete content
#[utoipa::path(
    delete,
    path = "/contents/{content_uuid}",
    params(
        ("content_uuid" = String, Path, description = "Content UUID", style = Simple, example = "fd2fe858-9962-404f-9174-c4f6f83cc39e")
    ),
    responses(
        (status = 204, description = "Delete successfully"),
        (status = 400, description = "Content Not Found", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Content",
)]
#[delete("/contents/{content_uuid}")]
pub async fn delete_content(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let content_uuid = path.into_inner();
        match Service::delete_content(&pool, content_uuid).await {
            Ok(size) if size > 0 => HttpResponse::NoContent().json(""),
            Ok(_) => HttpResponse::BadRequest().json(ResponseMessage {
                message: String::from("Content not found"),
            }),
            Err(e) => {
                if e == diesel::result::Error::NotFound {
                    HttpResponse::BadRequest().json(ResponseMessage {
                        message: String::from("Content not found"),
                    })
                } else {
                    println!("Failed to delete content: {}", e);
                    HttpResponse::InternalServerError().json(ResponseMessage {
                        message: String::from("Internal Server Error"),
                    })
                }
            }
        }
    } else {
        HttpResponse::Unauthorized().json(ResponseMessage {
            message: String::from("Unauthorized"),
        })
    }
}
