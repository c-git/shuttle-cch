use actix_web::web::{self, ServiceConfig};

mod day01;
mod day_minus_1;

pub fn modify_service_config(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(day_minus_1::task1_everything_is_ok));
    cfg.service(day_minus_1::scope());
    cfg.service(day01::scope());
}
