use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Json},
    HttpResponse,
};
use cookie::Cookie;

use crate::{
    services::database::Database,
    services::models::{
        auth::{AuthStore, Login},
        user::{InputUser, UserStore},
    },
    utils::{config::Config, errors::AppErrors},
};

#[post("/login")]
async fn login(
    db: web::Data<Database>,
    config: web::Data<Config>,
    login: web::Json<Login>,
) -> Result<HttpResponse, AppErrors> {
    let login = login.into_inner();

    let token = db.login(login, &config).await?;

    let cookie = Cookie::new("session", token);

    let mut result = HttpResponse::new(StatusCode::OK);
    result.add_cookie(&cookie)?;
    Ok(result)
}

#[post("/register")]
async fn register(
    db: web::Data<Database>,
    config: web::Data<Config>,
    create: web::Json<InputUser>,
) -> Result<HttpResponse, AppErrors> {
    let input = create.into_inner();
    db.create_user(input, &config).await?;
    Ok(HttpResponse::new(StatusCode::OK))
}
