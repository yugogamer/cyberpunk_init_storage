use actix_web::cookie::Cookie;
use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self},
    HttpResponse,
};
use actix_web::{route, HttpRequest, Responder};

use crate::controller::extract_token;
use crate::services::database::Database;
use crate::services::models::query::Schema;
use crate::utils::config::Config;

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

#[get("/graphql")]
pub async fn graphql_read(schema: web::Data<Schema>) -> impl Responder {
    let schema_string = schema.as_schema_language();

    HttpResponse::Ok().body(schema_string)
}
