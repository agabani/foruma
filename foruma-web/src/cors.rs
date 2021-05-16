use crate::configuration::Configuration;
use actix_web::http::Method;
use actix_web::{HttpRequest, HttpResponseBuilder};

pub trait Cors {
    fn insert_preflight_access_control_headers(&mut self, methods: &[Method]) -> &mut Self;
    fn insert_access_control_headers(
        &mut self,
        configuration: &Configuration,
        http_request: &HttpRequest,
    ) -> &mut Self;
}

impl Cors for HttpResponseBuilder {
    fn insert_preflight_access_control_headers(&mut self, methods: &[Method]) -> &mut Self {
        let methods = methods
            .iter()
            .map(|method| method.as_str())
            .collect::<Vec<_>>()
            .join(",");

        self.insert_header(("Access-Control-Allow-Headers", "content-type"))
            .insert_header(("Access-Control-Allow-Methods", methods))
    }

    fn insert_access_control_headers(
        &mut self,
        configuration: &Configuration,
        http_request: &HttpRequest,
    ) -> &mut Self {
        if let Some(origin) = http_request.headers().get("Origin") {
            if let Some(cors) = &configuration.cors {
                if let Ok(origin) = origin.to_str() {
                    if cors.origins.iter().any(|o| o == origin) {
                        return self
                            .insert_header(("Access-Control-Allow-Credentials", "true"))
                            .insert_header(("Access-Control-Allow-Origin", origin));
                    }
                }
            }
        }

        self
    }
}
