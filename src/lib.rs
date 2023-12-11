use actix_web::web::{self, ServiceConfig};

mod day_minus_1;

pub fn modify_service_config(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(day_minus_1::day_minus_1_task1));
}
