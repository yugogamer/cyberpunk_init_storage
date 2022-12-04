use actix_web::web::{self, Data};
use actix_web::{get, middleware, App, HttpResponse, HttpServer};

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
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(config))
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/api").service(
                    web::scope("/account")
                        .service(controller::account::login)
                        .service(controller::account::register),
                ),
            )
            .service(index)
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("Listening on http://{}:{}", config.host, config.port);
    server.await
}
