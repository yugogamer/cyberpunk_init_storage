use actix_web::cookie::Cookie;
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self},
    HttpResponse,
};

use crate::services::models::database::DatabaseTrait;
use crate::{
    services::database::Database,
    services::models::{auth::Login, user::InputUser},
    utils::{config::Config, errors::AppErrors},
};

#[post("/login")]
async fn login(
    db: web::Data<Database>,
    config: web::Data<Config>,
    login: web::Json<Login>,
) -> Result<HttpResponse, AppErrors> {
    let login = login.into_inner();

    let token = db.auth_service().login(login, &config).await?;

    let mut cookie = Cookie::new("session", token);
    cookie.set_path("/");

    let mut result = HttpResponse::new(StatusCode::OK);
    result.add_cookie(&cookie)?;
    Ok(result)
}

#[get("/logout")]
async fn logout() -> Result<HttpResponse, AppErrors> {
    let mut result = HttpResponse::new(StatusCode::OK);

    let mut cookie = Cookie::new("session", "");
    cookie.set_path("/");
    result.add_removal_cookie(&cookie)?;
    Ok(result)
}

#[post("/register")]
async fn register(
    db: web::Data<Database>,
    config: web::Data<Config>,
    create: web::Json<InputUser>,
) -> Result<HttpResponse, AppErrors> {
    let input = create.into_inner();
    db.user_store().create_user(input, &config).await?;
    Ok(HttpResponse::new(StatusCode::OK))
}
