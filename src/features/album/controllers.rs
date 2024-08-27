use actix_multipart::Multipart;
use actix_web::{delete, HttpRequest, HttpResponse, post, Responder, web};

use crate::common::database::DbPool;
use crate::common::enums::Role::Admin;
use crate::common::models::response_message::ResponseMessage;
use crate::common::utils::{delete_directory_if_exists, parse_payload_data};
use crate::features::album::models::{CreateAlbumRequest, GetAlbumRequest, UpdateAlbumRequest};
use crate::features::album::services::Service;
use crate::features::check_role;

/// Create Album
///
/// Create a new album
#[utoipa::path(
    post,
    path = "/album",
    request_body(
        content = CreateAlbumRequest,
        description = "Create Album",
        content_type = "multipart/form-data",
    ),
    responses(
        (status = 201, description = "Created successfully", body = Album),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Album",
)]
#[post("/album")]
pub async fn create_album(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    req: web::Form<CreateAlbumRequest>,
    payload: Multipart,
) -> impl Responder {
    if check_role(http_request) == Admin {
        match parse_payload_data::<CreateAlbumRequest>(payload).await {
            Ok((req_data, tmp_path)) => {
                match Service::create_album(&pool, req_data).await {
                    Ok(new_album) => {
                        delete_directory_if_exists(&tmp_path);
                        HttpResponse::Created().json(new_album)
                    }
                    Err(e) => {
                        println!("Failed to create album: {}", e);
                        delete_directory_if_exists(&tmp_path);
                        HttpResponse::InternalServerError().json(ResponseMessage {
                            message: String::from("Internal Server Error"),
                        })
                    }
                }
            }
            Err(e) => e,
        }
    } else {
        HttpResponse::Unauthorized().json(ResponseMessage {
            message: String::from("Unauthorized"),
        })
    }
}

/// Get Albums
///
/// Get album list
#[utoipa::path(
    post,
    path = "/albums",
    request_body = GetAlbumRequest,
    responses(
    (status = 200, description = "Request successfully", body = ResponseData < Album >),
    (status = 401, description = "Unauthorized error", body = ResponseMessage),
    (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
    ("api_key" = [])
    ),
    tag = "Album",
)]
#[post("/albums")]
pub async fn get_albums(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    req: web::Json<GetAlbumRequest>,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let album_filters = req.into_inner();
        match Service::get_albums(&pool, album_filters).await {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(err) => HttpResponse::BadRequest().json(ResponseMessage {
                message: err.to_string(),
            }),
        }
    } else {
        HttpResponse::Unauthorized().json(ResponseMessage {
            message: String::from("Unauthorized"),
        })
    }
}

/// Update Album
///
/// Update album
#[utoipa::path(
    post,
    path = "/album/{album_uuid}",
    request_body(
        content = UpdateAlbumRequest,
        description = "Update Album",
        content_type = "multipart/form-data",
    ),
    responses(
        (status = 200, description = "Update successfully"),
        (status = 400, description = "Update failed", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Album",
)]
#[post("/{album_uuid}")]
pub async fn update_album(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    path: web::Path<String>,
    payload: Multipart,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let album_uuid = path.into_inner();
        match parse_payload_data::<UpdateAlbumRequest>(payload).await {
            Ok((req_data, tmp_path)) => {
                match Service::update_album(&pool, album_uuid, req_data).await {
                    Ok(new_album) => {
                        HttpResponse::Created().json(new_album)
                    }
                    Err(e) => {
                        println!("Failed to update album: {}", e);
                        delete_directory_if_exists(&tmp_path);
                        HttpResponse::BadRequest().json(ResponseMessage {
                            message: String::from("Failed to update album"),
                        })
                    }
                }
            }
            Err(e) => e,
        }
    } else {
        HttpResponse::Unauthorized().json(ResponseMessage {
            message: String::from("Unauthorized"),
        })
    }
}

/// Delete Album
///
/// Delete album
#[utoipa::path(
    delete,
    path = "/album/{album_uuid}",
    params(
        ("album_uuid" = i32, Path, description = "Album ID", style = Simple, example = "fd2fe858-9962-404f-9174-c4f6f83cc39e")
    ),
    responses(
        (status = 204, description = "Delete successfully"),
        (status = 400, description = "Delete Not Found", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Album",
)]
#[delete("/{album_uuid}")]
pub async fn delete_album(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let album_uuid = path.into_inner();
        match Service::delete_album(&pool, album_uuid).await {
            Ok(size) if size > 0 => HttpResponse::NoContent().json(""),
            Ok(_) => HttpResponse::BadRequest().json(ResponseMessage {
                message: String::from("Nothing Deleted"),
            }),
            Err(e) => {
                println!("Failed to delete album: {}", e);
                HttpResponse::InternalServerError().json(ResponseMessage {
                    message: String::from("Internal Server Error"),
                })
            }
        }
    } else {
        HttpResponse::Unauthorized().json(ResponseMessage {
            message: String::from("Unauthorized"),
        })
    }
}
