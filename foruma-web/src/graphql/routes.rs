use crate::graphql::model::ForumaSchema;
use actix_web::{guard, web, HttpRequest, HttpResponse};
use async_graphql::{http::playground_source, http::GraphQLPlaygroundConfig, Request};

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
    _http_request: HttpRequest,
    request: web::Json<Request>,
) -> HttpResponse {
    let request = request.0;

    let response = schema.execute(request).await;

    HttpResponse::Ok().json(response)
}
