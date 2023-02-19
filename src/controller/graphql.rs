use actix_web::Responder;
use actix_web::{
    get, post,
    web::{self},
    HttpResponse,
};

use crate::services::database::Database;
use crate::services::models::auth::LightUser;
use crate::services::models::query::Schema;

pub struct GraphqlContext {
    pub db: Database,
    pub user_id: i32,
}

impl juniper::Context for GraphqlContext {}

#[post("/graphql")]
pub async fn graphql(
    user: LightUser,
    pool: web::Data<Database>,
    schema: web::Data<Schema>,
    data: web::Json<juniper::http::GraphQLRequest>,
) -> impl Responder {
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
