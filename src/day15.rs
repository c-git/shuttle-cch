use actix_web::{
    error,
    web::{self, Json},
    HttpResponse, Scope,
};
use serde::Deserialize;
use tracing::info;
pub(crate) fn scope() -> Scope {
    web::scope("/15").route("/nice", web::post().to(task1_naughty_or_nice_strings))
}

enum StringType {
    Nice,
    Naughty,
    BadRequest,
}

#[derive(Debug, Deserialize)]
struct Input {
    input: String,
}

#[tracing::instrument]
async fn task1_naughty_or_nice_strings(
    Json(Input { input: password }): web::Json<Input>,
) -> HttpResponse {
    info!("{password}");
    HttpResponse::Ok().finish()
}
