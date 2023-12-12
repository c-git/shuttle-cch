use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use tracing::error;

mod day01;
mod day_minus_1;

pub fn modify_service_config(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(day_minus_1::task1_everything_is_ok));
    cfg.service(day_minus_1::scope());
    cfg.service(day01::scope());
    cfg.route("/{path:.*}", web::get().to(catch_unmatched));
    cfg.default_service(web::route().to(not_found));
}

#[tracing::instrument]
async fn catch_unmatched(path: web::Path<String>) -> actix_web::Result<HttpResponse> {
    error!("Caught an unmatched route");
    Ok(HttpResponse::NotFound().body("404 Not found (Logged)\n"))
}

#[tracing::instrument]
pub async fn not_found() -> actix_web::Result<HttpResponse> {
    error!("Failed to match route");
    Ok(HttpResponse::NotFound().body("404 - Not found\n"))
}
