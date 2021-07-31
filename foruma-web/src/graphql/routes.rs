use actix_web::{guard, web, HttpRequest, HttpResponse};
use async_graphql::{http::playground_source, http::GraphQLPlaygroundConfig, Request};

use crate::graphql::model::GraphQlSchema;
use crate::http_request_ext::HttpRequestExt;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").guard(guard::Get()).to(index_playground))
        .service(web::resource("/").guard(guard::Post()).to(index));
}

#[allow(clippy::unused_async)]
pub async fn index_playground() -> Result<actix_web::HttpResponse, actix_web::Error> {
    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/api/graphql/").subscription_endpoint("/api/graphql/"),
        )))
}

pub async fn index(
    schema: web::Data<GraphQlSchema>,
    http_request: HttpRequest,
    request: web::Json<Request>,
) -> HttpResponse {
    let mut request = request.0;

    if let Some(ip_address) = http_request.client_ip() {
        request = request.data(ip_address);
    }

    if let Some(session_id) = http_request.session_id() {
        request = request.data(session_id);
    }

    if let Some(user_agent) = http_request.user_agent() {
        request = request.data(user_agent);
    }

    let response = schema.execute(request).await;

    let mut http_response = HttpResponse::Ok();

    for (key, value) in &response.http_headers {
        http_response.append_header((key.to_string(), value.to_string()));
    }

    http_response.json(response)
}
