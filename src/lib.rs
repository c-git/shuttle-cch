use actix_web::{
    middleware::Logger,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse,
};
use tracing::error;

mod day01;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day11;
mod day12;
mod day15;
mod day20;
mod day21;
mod day_minus_1;

pub fn modify_service_config(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(day_minus_1::task1_everything_is_ok));
    cfg.service(day_minus_1::scope().wrap(Logger::default()));
    cfg.service(day01::scope().wrap(Logger::default()));
    cfg.service(day04::scope().wrap(Logger::default()));
    cfg.service(day05::scope().wrap(Logger::default()));
    cfg.service(day06::scope().wrap(Logger::default()));
    cfg.service(day07::scope().wrap(Logger::default()));
    cfg.service(day08::scope().wrap(Logger::default()));
    cfg.service(day11::scope().wrap(Logger::default()));
    cfg.service(day12::scope().wrap(Logger::default()));
    cfg.service(day15::scope().wrap(Logger::default()));
    cfg.service(day20::scope().wrap(Logger::default()));
    cfg.service(day21::scope().wrap(Logger::default()));
    cfg.default_service(web::route().to(not_found).wrap(Logger::default()));
}

#[tracing::instrument]
pub async fn not_found(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    error!("Failed to match route");
    Ok(HttpResponse::NotFound().body("404 - Not found\n"))
}
