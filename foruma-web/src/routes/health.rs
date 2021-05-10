use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("liveness", web::get().to(get))
        .route("readiness", web::get().to(get));
}

async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}
