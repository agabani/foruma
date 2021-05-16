use crate::configuration::Configuration;
use actix_web::{http::Method, web, HttpRequest, HttpResponse, HttpResponseBuilder};
use cookie::{CookieBuilder, CookieJar, Key, SameSite};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/login")
            .route("", web::method(Method::OPTIONS).to(login_option))
            .route("", web::method(Method::POST).to(login_post)),
    )
    .service(
        web::scope("/logout")
            .route("", web::method(Method::OPTIONS).to(logout_option))
            .route("", web::method(Method::POST).to(logout_post)),
    )
    .service(
        web::scope("/signup")
            .route("", web::method(Method::OPTIONS).to(signup_option))
            .route("", web::method(Method::POST).to(signup_post)),
    )
    .service(
        web::scope("/whoami")
            .route("", web::method(Method::OPTIONS).to(whoami_option))
            .route("", web::method(Method::GET).to(whoami_get)),
    );
}

trait AccessControlHeaders {
    fn insert_preflight_access_control_headers(&mut self, methods: &[Method]) -> &mut Self;
    fn insert_access_control_headers(
        &mut self,
        configuration: &Configuration,
        http_request: &HttpRequest,
    ) -> &mut Self;
}

impl AccessControlHeaders for HttpResponseBuilder {
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

#[tracing::instrument(skip(configuration))]
fn login_option(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
) -> HttpResponse {
    tracing::info!("http request");

    HttpResponse::Ok()
        .insert_access_control_headers(configuration.get_ref(), &http_request)
        .insert_preflight_access_control_headers(&[Method::POST])
        .finish()
}

#[derive(serde::Deserialize)]
struct LoginPostRequest {
    username: String,
    password: String,
}

#[tracing::instrument(skip(configuration, key, request))]
fn login_post(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
    key: web::Data<Key>,
    request: web::Json<LoginPostRequest>,
) -> HttpResponse {
    tracing::info!("http request");

    return if request.username == "test-username" && request.password == "test-password" {
        let key = key.get_ref();

        let cookie = CookieBuilder::new("authentication", request.username.clone())
            .http_only(true)
            .same_site(SameSite::None)
            .secure(true)
            .path("/")
            .finish();

        let mut jar = CookieJar::new();
        jar.private_mut(&key).add(cookie);

        let plain = jar.get("authentication").cloned().unwrap();

        HttpResponse::Ok()
            .cookie(plain)
            .insert_access_control_headers(configuration.get_ref(), &http_request)
            .finish()
    } else {
        HttpResponse::Unauthorized()
            .insert_access_control_headers(configuration.get_ref(), &http_request)
            .finish()
    };
}

#[tracing::instrument(skip(configuration))]
fn logout_option(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
) -> HttpResponse {
    tracing::info!("http request");

    HttpResponse::Ok()
        .insert_access_control_headers(configuration.get_ref(), &http_request)
        .insert_preflight_access_control_headers(&[Method::POST])
        .finish()
}

#[tracing::instrument(skip(configuration, key))]
fn logout_post(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
    key: web::Data<Key>,
) -> HttpResponse {
    tracing::info!("http request");

    let key = key.get_ref();

    let option = http_request.cookie("authentication");
    if option.is_none() {
        return HttpResponse::Unauthorized()
            .insert_access_control_headers(configuration.get_ref(), &http_request)
            .finish();
    }

    let cookie = option.unwrap();
    let mut jar = CookieJar::new();

    let option = jar.private_mut(&key).decrypt(cookie);
    if option.is_none() {
        return HttpResponse::Unauthorized()
            .insert_access_control_headers(configuration.get_ref(), &http_request)
            .finish();
    }
    let mut cookie = option.unwrap();
    cookie.set_http_only(true);
    cookie.set_secure(true);

    return HttpResponse::Ok()
        .insert_access_control_headers(configuration.get_ref(), &http_request)
        .del_cookie(&cookie)
        .finish();
}

#[tracing::instrument(skip(configuration))]
fn signup_option(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
) -> HttpResponse {
    tracing::info!("http request");

    HttpResponse::Ok()
        .insert_access_control_headers(configuration.get_ref(), &http_request)
        .insert_preflight_access_control_headers(&[Method::POST])
        .finish()
}

#[derive(serde::Deserialize)]
struct SignupPostRequest {
    username: String,
    // password: String,
}

#[tracing::instrument(skip(configuration, key, request))]
fn signup_post(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
    key: web::Data<Key>,
    request: web::Json<SignupPostRequest>,
) -> HttpResponse {
    tracing::info!("http request");

    let cookie = CookieBuilder::new("authentication", request.username.clone())
        .http_only(true)
        .same_site(SameSite::None)
        .secure(true)
        .path("/")
        .finish();

    let mut jar = CookieJar::new();
    jar.private_mut(&key).add(cookie);

    let plain = jar.get("authentication").cloned().unwrap();

    HttpResponse::Ok()
        .cookie(plain)
        .insert_access_control_headers(configuration.get_ref(), &http_request)
        .finish()
}

#[tracing::instrument(skip(configuration))]
fn whoami_option(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
) -> HttpResponse {
    tracing::info!("http request");

    HttpResponse::Ok()
        .insert_access_control_headers(configuration.get_ref(), &http_request)
        .insert_preflight_access_control_headers(&[Method::GET])
        .finish()
}

#[derive(serde::Serialize)]
struct WhoAmIResponse {
    username: String,
}

#[tracing::instrument(skip(configuration, key))]
fn whoami_get(
    http_request: HttpRequest,
    configuration: web::Data<Configuration>,
    key: web::Data<Key>,
) -> HttpResponse {
    tracing::info!("http request");

    let key = key.get_ref();

    let option = http_request.cookie("authentication");
    if option.is_none() {
        return HttpResponse::Unauthorized()
            .insert_access_control_headers(configuration.get_ref(), &http_request)
            .finish();
    }

    let cookie = option.unwrap();
    let mut jar = CookieJar::new();

    let option = jar.private_mut(&key).decrypt(cookie);
    if option.is_none() {
        return HttpResponse::Unauthorized()
            .insert_access_control_headers(configuration.get_ref(), &http_request)
            .finish();
    }
    let cookie = option.unwrap();

    return HttpResponse::Ok()
        .insert_access_control_headers(configuration.get_ref(), &http_request)
        .json(WhoAmIResponse {
            username: cookie.value().to_string(),
        });
}
