use actix_web::{error, web, HttpRequest, HttpResponse, Scope};
use base64::{engine::general_purpose, Engine};
use tracing::info;
pub(crate) fn scope() -> Scope {
    web::scope("/7").route("/decode", web::get().to(task1_based_encoding_64_th_edition))
}

#[tracing::instrument]
async fn task1_based_encoding_64_th_edition(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let cookie = match req.cookie("recipe") {
        Some(val) => val,
        None => return Err(error::ErrorBadRequest("recipe cookie not found")),
    };
    info!("Incoming Cookie: {cookie}");

    let value = cookie.value();
    let decoded = general_purpose::STANDARD
        .decode(value)
        .map_err(error::ErrorBadRequest)?;
    let json = String::from_utf8(decoded).map_err(error::ErrorBadRequest)?;
    info!(json);

    Ok(HttpResponse::Ok().body(json))
}
