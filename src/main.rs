use actix_cors::Cors;
use actix_web::web::{self, Data};
use actix_web::{get, http, middleware, App, HttpResponse, HttpServer};
use juniper::RootNode;

use crate::services::models::query::{create_schema, Mutation, Query};

mod controller;
mod services;
mod utils;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("status : Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = utils::config::Config::new();
    if let Err(e) = config {
        println!("{}", e);
        std::process::exit(1);
    }
    let config = config.unwrap();
    let pool = services::database::Database::new(&config).await;
    let pool = match pool {
        Ok(pool) => pool,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    };

    let server = HttpServer::new(move || {
        let config = utils::config::Config::new().unwrap();
        let session_header = http::header::HeaderName::from_lowercase(b"session").unwrap();
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("https://studio.apollographql.com")
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
                    .service(controller::account::graphql)
                    .service(controller::account::graphql_read),
            )
            .service(index)
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("Listening on http://{}:{}", config.host, config.port);
    server.await
}
