use crate::telemetry::TraceErrorExt;
use actix_web::{web, HttpResponse};
use sqlx::{Pool, Postgres};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("liveness", web::get().to(liveness_get))
        .route("readiness", web::get().to(readiness_get));
}

fn liveness_get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn readiness_get(postgres_pool: web::Data<Pool<Postgres>>) -> HttpResponse {
    postgres(postgres_pool.get_ref()).await.map_or_else(
        |_| HttpResponse::InternalServerError().finish(),
        |_| HttpResponse::Ok().finish(),
    )
}

async fn postgres(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let _: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool)
        .await
        .trace_err()?;
    Ok(())
}
