use crate::common::database::DbPool;
use crate::common::enums::Role::Admin;
use crate::common::models::response_message::ResponseMessage;
use crate::features::check_role;
use crate::features::category::models;
use crate::features::category::services::Service;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use crate::common::ne_parse::NEParse;
use crate::features::category::models::UpdateCategoryRequest;

/// Add Category
///
/// Add category
#[utoipa::path(
    post,
    path = "/category",
    request_body(
        content = AddCategoryRequest,
        description = "Add Episode Contents Request",
    ),
    responses(
        (status = 201, description = "Add successfully", body = CategoryResponse),
        (status = 400, description = "Update failed", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Category",
)]
#[post("/category")]
pub async fn add_category(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    req: web::Json<models::AddCategoryRequest>,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let request = req.into_inner();
        match Service::add_category(&pool, request).await {
            Ok(msg) => {
                HttpResponse::Created().json(ResponseMessage {
                    message: String::from(msg),
                })
            }
            Err(e) => {
                HttpResponse::BadRequest().json(ResponseMessage {
                    message: String::from(e),
                })
            }
        }
    } else {
        HttpResponse::Unauthorized().json(ResponseMessage {
            message: String::from("Unauthorized"),
        })
    }
}

/// Get Categories
///
/// Get categories
#[utoipa::path(
    get,
    path = "/categories",
    responses(
        (status = 200, description = "Get successfully", body = ResponseDataCategory),
        (status = 400, description = "Update failed", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Category",
)]
#[get("/categories")]
pub async fn get_categories(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
) -> impl Responder {
    if check_role(http_request) == Admin {
        match Service::get_categories(&pool).await {
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

/// Update Category
///
/// Update category
#[utoipa::path(
    put,
    path = "/category",
    request_body(
        content = UpdateCategoryRequest,
        description = "Update Content Request",
    ),
    responses(
        (status = 200, description = "Add successfully", body = CategoryResponse),
        (status = 400, description = "Update failed", body = ResponseMessage),
        (status = 401, description = "Unauthorized error", body = ResponseMessage),
        (status = 500, description = "Internal server error", body = ResponseMessage)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Category",
)]
#[put("/category")]
pub async fn update_category(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    req: web::Json<UpdateCategoryRequest>,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let update_request = req.into_inner();
        match Service::update_category(&pool, update_request).await {
            Ok(response) => {
                HttpResponse::Ok().json(response)
            }
            Err(_) => {
                HttpResponse::BadRequest().json(ResponseMessage {
                    message: String::from("Failed to update category"),
                })
            }
        }
    } else {
        HttpResponse::Unauthorized().json(ResponseMessage {
            message: String::from("Unauthorized"),
        })
    }
}


/// Delete Category
///
/// Delete category
#[utoipa::path(
    delete,
    path = "/category/{category_id}",
    params(
        ("category_id" = String, Path, description = "Category ID", style = Simple, example = "1")
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
    tag = "Category",
)]
#[delete("/category/{category_id}")]
pub async fn delete_category(
    pool: web::Data<DbPool>,
    http_request: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    if check_role(http_request) == Admin {
        let string_id = path.into_inner();
        let c_id : i32 = NEParse::opt_immut_str_to_opt_i32(Some(string_id.as_str())).unwrap_or(0i32);
        match Service::delete_category(&pool, c_id).await {
            Ok(_) => HttpResponse::NoContent().json(""),
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
