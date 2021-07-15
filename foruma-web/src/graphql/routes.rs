use crate::context::Context;
use crate::graphql::model::GraphQlSchema;
use crate::http_request_ext::HttpRequestExt;
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
            GraphQLPlaygroundConfig::new("/api/graphql/").subscription_endpoint("/api/graphql/"),
        )))
}

pub async fn index(
    context: web::Data<Context>,
    schema: web::Data<GraphQlSchema>,
    http_request: HttpRequest,
    request: web::Json<Request>,
) -> HttpResponse {
    let mut request = request.0.data(context);

    if let Some(session_id) = http_request.session_id() {
        request = request.data(session_id);
    }

    let response = schema.execute(request).await;

    HttpResponse::Ok().json(response)
}
