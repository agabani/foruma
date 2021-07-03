use crate::graphql::model::ForumaSchema;
use actix_web::{guard, web};
use async_graphql::{http::playground_source, http::GraphQLPlaygroundConfig};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").guard(guard::Get()).to(index_playground))
        .service(web::resource("/").guard(guard::Post()).to(index));
}

pub async fn index_playground() -> Result<actix_web::HttpResponse, actix_web::Error> {
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

pub async fn index(
    schema: web::Data<ForumaSchema>,
    req: async_graphql_actix_web::Request,
) -> async_graphql_actix_web::Response {
    schema.execute(req.into_inner()).await.into()
}
