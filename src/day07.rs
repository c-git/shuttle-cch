use actix_web::{web, HttpRequest, HttpResponse, Scope};
use tracing::info;
pub(crate) fn scope() -> Scope {
    web::scope("/7").route("/decode", web::get().to(task1_based_encoding_64_th_edition))
}

#[tracing::instrument]
async fn task1_based_encoding_64_th_edition(req: HttpRequest) -> HttpResponse {
    let cookie = match req.cookie("recipe") {
        Some(val) => val,
        None => return HttpResponse::BadRequest().body("recipe cookie not found"),
    };
    info!("Incoming Cookie: {cookie}");
    HttpResponse::NotImplemented().finish()
}
