extern crate core;

use std::env;

use crate::common::middleware::response_time_middleware::ResponseTime;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{middleware, middleware::Logger, web, App, HttpServer};
use actix_web_lab::middleware::CatchPanic;
use dotenvy::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::common::utils::{notfound_404};

mod common;
mod features;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // ! Database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = common::database::sqlite_connection::connect(database_url);

    // ! Swagger
    let openapi = features::ApiDoc::openapi();

    // ! API Keys
    let admin_key = env::var("ADMIN_API_KEY").expect("Admin API Key must be set");
    let user_key = env::var("USER_API_KEY").expect("Admin API Key must be set");
    let public_routes = vec![String::from("/health")];
    // Log that the API is starting
    println!("📔API Documentation can be found at ➡️ http://localhost:8010/swagger/index.html");
    HttpServer::new(move || {
        // ! Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(ResponseTime)
            .wrap(
                common::middleware::api_key_middleware::ApiKeyMiddleware::new(
                    admin_key.clone(),
                    user_key.clone(),
                    public_routes.clone(),
                ),
            )
            .wrap(cors)
            .app_data(web::Data::new(connection.clone()))
            .configure(features::config_routes)
            .wrap(CatchPanic::default())
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(Files::new("/static", ".").default_handler(web::to(notfound_404)) )
    })
    .bind(("127.0.0.1", 8010))?
    .run()
    .await
}
