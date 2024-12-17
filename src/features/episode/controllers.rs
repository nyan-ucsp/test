use crate::common::database::DbPool;
use crate::common::enums::Role::Admin;
use crate::common::models::response_message::ResponseMessage;
use crate::common::utils::{delete_directory_if_exists, parse_payload_data};
use crate::features::check_role;
use crate::features::episode::models;
use crate::features::episode::models::UpdateEpisodeRequest;
use crate::features::episode::services::Service;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};

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
        (status = 400, description = "Bad Request", body = ResponseMessage),
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
                let check_required_fields =
                    models::CreateEpisodeRequest::check_required_data(payload_data.clone()).await;
                if check_required_fields {
                    let req_data =
                        models::CreateEpisodeRequest::from_payload_data(payload_data).await;
                    match Service::create_episode(&pool, req_data).await {
                        Ok(new_episode) => {
                            delete_directory_if_exists(&tmp_path);
                            HttpResponse::Created().json(new_episode)
                        }
                        Err(e) => {
                            delete_directory_if_exists(&tmp_path);
                            HttpResponse::BadRequest().json(ResponseMessage {
                                message: String::from(e),
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

/// Get Episodes By Album Id
///
/// Get episodes by album id
#[utoipa::path(
    post,
    path = "/episodes/{album_id}",
    request_body = FilterEpisodeRequest,
    responses(
        (status = 200, description = "Request successfully", body = ResponseDataEpisode),
        (status = 400, description = "Album not found", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
    ("api_key" = [])
    ),
    tag = "Episode",
)]
#[post("/episodes/{album_id}")]
pub async fn get_episodes_by_album_id(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    req: web::Json<models::FilterEpisodeRequest>,
    path: web::Path<String>,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let album_id_string = path.into_inner();
        match album_id_string.parse::<i32>() {
            Ok(id) => {
                let episode_filters = req.into_inner();
                match Service::get_episodes_by_album_id(&pool, id, episode_filters).await {
                    Ok(data) => HttpResponse::Ok().json(data),
                    Err(err) => HttpResponse::BadRequest().json(ResponseMessage {
                        message: err.to_string(),
                    }),
                }
            }
            Err(_) => HttpResponse::BadRequest().json(ResponseMessage {
                message: String::from("Invalid album ID"),
            }),
        }
    } else {
        HttpResponse::Unauthorized().json(ResponseMessage {
            message: String::from("Unauthorized"),
        })
    }
}

/// Get Episode
///
/// Get episode by episode uuid
#[utoipa::path(
    get,
    path = "/episode/{episode_uuid}",
    params(
        ("episode_uuid" = String, Path, description = "Episode UUID", style = Simple, example = "fd2fe858-9962-404f-9174-c4f6f83cc39e")
    ),
    responses(
        (status = 200, description = "Get successfully", body = EpisodeResponse),
        (status = 400, description = "Episode Not Found", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Episode",
)]
#[get("/episode/{episode_uuid}")]
pub async fn get_episode(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let episode_uuid = path.into_inner();
        match Service::get_episode_by_episode_uuid(&pool, episode_uuid).await {
            Ok(episode)=>HttpResponse::Ok().json(episode),
            Err(e) => {
                if e == diesel::result::Error::NotFound {
                    HttpResponse::BadRequest().json(ResponseMessage {
                        message: String::from("Episode not found"),
                    })
                } else {
                    println!("Failed to get episode: {}", e);
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

/// Update Episode
///
/// Update episode
#[utoipa::path(
    put,
    path = "/episodes/{episode_uuid}",
    request_body(
        content = UpdateEpisodeRequest,
        description = "Update Episode",
        content_type = "multipart/form-data",
    ),
    responses(
        (status = 200, description = "Update successfully", body = EpisodeResponse),
        (status = 400, description = "Update failed", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Episode",
)]
#[put("/episodes/{episode_uuid}")]
pub async fn update_episode(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    path: web::Path<String>,
    payload: Multipart,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let episode_uuid = path.into_inner();
        match parse_payload_data(payload).await {
            Ok((payload_data, tmp_path)) => {
                let req_data = UpdateEpisodeRequest::from_payload_data(payload_data).await;
                match Service::update_episode(&pool, episode_uuid, req_data).await {
                    Ok(updated_episode) => {
                        delete_directory_if_exists(&tmp_path);
                        HttpResponse::Ok().json(updated_episode)
                    }
                    Err(e) => {
                        println!("Failed to update episode: {}", e);
                        delete_directory_if_exists(&tmp_path);
                        HttpResponse::BadRequest().json(ResponseMessage {
                            message: String::from("Failed to update episode"),
                        })
                    }
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

/// Delete Episode
///
/// Delete episode
#[utoipa::path(
    delete,
    path = "/episode/{episode_uuid}",
    params(
        ("episode_uuid" = String, Path, description = "Episode UUID", style = Simple, example = "fd2fe858-9962-404f-9174-c4f6f83cc39e")
    ),
    responses(
        (status = 204, description = "Delete successfully"),
        (status = 400, description = "Episode Not Found", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Episode",
)]
#[delete("/episode/{episode_uuid}")]
pub async fn delete_episode(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let episode_uuid = path.into_inner();
        match Service::delete_episode(&pool, episode_uuid).await {
            Ok(size) if size > 0 => HttpResponse::NoContent().json(""),
            Ok(_) => HttpResponse::BadRequest().json(ResponseMessage {
                message: String::from("Episode not found"),
            }),
            Err(e) => {
                if e == diesel::result::Error::NotFound {
                    HttpResponse::BadRequest().json(ResponseMessage {
                        message: String::from("Episode not found"),
                    })
                } else {
                    println!("Failed to delete episode: {}", e);
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
