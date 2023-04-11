use actix_cors::Cors;
use actix_web::web::{self, Data};
use actix_web::{get, http, middleware, App, HttpResponse, HttpServer};

use crate::services::bucket::BucketHandler;
use crate::services::models::database::DatabaseTrait;
use crate::services::models::query::create_schema;

mod controller;
mod services;
mod utils;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("status : Ok")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = utils::config::Config::new();
    if let Err(e) = config {
        println!("error while loading config: {}", e);
        std::process::exit(1);
    }
    let config = config.unwrap();
    let pool = services::database::Database::new(&config).await;
    let pool = match pool {
        Ok(pool) => pool,
        Err(e) => {
            println!("Error while connecting to database : {:?}", e);
            std::process::exit(1);
        }
    };
    let storage = BucketHandler::new(&config).await;

    let server = HttpServer::new(move || {
        let config = utils::config::Config::new().unwrap();
        let session_header = http::header::HeaderName::from_lowercase(b"session").unwrap();
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("https://studio.apollographql.com")
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://127.0.0.1:5173")
            .allowed_origin("https://raina.ovh")
            .allowed_origin("https://www.cyberpunk.raina.ovh")
            .allowed_origin("https://cyberpunk-website.s3-website.fr-par.scw.cloud")
            .supports_credentials()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                session_header,
            ])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(config))
            .app_data(Data::new(storage.clone()))
            .app_data(Data::new(create_schema()))
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/account")
                            .service(controller::account::login)
                            .service(controller::account::register)
                            .service(controller::account::logout),
                    )
                    .service(controller::graphql::graphql)
                    .service(controller::graphql::graphql_read)
                    .service(controller::bot::roll),
            )
            .service(index)
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("Listening on http://{}:{}", config.host, config.port);
    server.await
}
