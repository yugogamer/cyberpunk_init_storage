use actix_web::cookie::Cookie;
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self},
    HttpResponse,
};
use actix_web::{route, HttpRequest, Responder};

use crate::services::models::query::Schema;
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
    result.add_removal_cookie(&cookie);
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

pub struct GraphqlContext {
    pub db: Database,
    pub user_id: i32,
}

impl juniper::Context for GraphqlContext {}

#[post("/graphql")]
pub async fn graphql(
    request: HttpRequest,
    pool: web::Data<Database>,
    schema: web::Data<Schema>,
    config: web::Data<Config>,
    data: web::Json<juniper::http::GraphQLRequest>,
) -> impl Responder {
    let token = extract_token(&request);
    if token.is_none() {
        return HttpResponse::Unauthorized().finish();
    }
    let user = crate::utils::auth::verify_jwt(&token.unwrap(), &config.jwt_secret);
    if user.is_err() {
        return HttpResponse::Unauthorized().finish();
    }
    let user = user.unwrap();

    let ctx = GraphqlContext {
        db: pool.get_ref().clone(),
        user_id: user.id,
    };
    let res = data.execute(&schema, &ctx).await;

    HttpResponse::Ok().json(res)
}

fn extract_token(request: &HttpRequest) -> Option<String> {
    let cookie_token = request.cookie("session");
    if cookie_token.is_none() {
        let header_token = request.headers().get("session");
        if header_token.is_none() {
            return None;
        }
        return Some(header_token.unwrap().to_str().unwrap().to_string());
    }
    let token = cookie_token.unwrap();
    Some(token.value().to_string())
}

#[get("/graphql")]
pub async fn graphql_read(schema: web::Data<Schema>) -> impl Responder {
    let schema_string = schema.as_schema_language();

    HttpResponse::Ok().body(schema_string)
}
