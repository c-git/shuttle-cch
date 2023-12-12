use actix_files::NamedFile;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{error::ErrorBadRequest, web, Error, HttpRequest, HttpResponse, Responder, Scope};
use anyhow::Context;

pub(crate) fn scope() -> Scope {
    web::scope("/11")
        .route(
            "/assets/decoration.png",
            web::get().to(task1_served_on_a_silver_platter),
        )
        .route("/red_pixels", web::post().to(task2_bull_mode_activated))
}

#[tracing::instrument]
async fn task1_served_on_a_silver_platter() -> actix_web::Result<NamedFile> {
    NamedFile::open_async("assets/decoration.png")
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
}

#[derive(Debug, MultipartForm)]
struct PostData {
    image: TempFile,
}

#[tracing::instrument(skip(payload))]
async fn task2_bull_mode_activated(
    MultipartForm(payload): MultipartForm<PostData>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder, Error> {
    let img = image::open(payload.image.file)
        .context("Failed to load image")
        .map_err(ErrorBadRequest)?;
    let pixels = match img.as_rgb8() {
        Some(value) => value,
        None => return Err(ErrorBadRequest("Unable to access image pixels")),
    };
    dbg!(pixels);

    Ok(HttpResponse::Ok())
}
