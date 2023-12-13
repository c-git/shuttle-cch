use actix_files::NamedFile;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{error::ErrorBadRequest, web, Error, HttpRequest, HttpResponse, Responder, Scope};
use image::Rgb;
use tracing::info;

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
    let img = image::io::Reader::open(payload.image.file)?
        .with_guessed_format()?
        .decode()
        .map_err(ErrorBadRequest)?;
    let pixels = match img.as_rgb8() {
        Some(value) => value.pixels(),
        None => return Err(ErrorBadRequest("Unable to access image pixels")),
    };
    let result: usize = pixels
        .map(|Rgb([red, green, blue])| {
            if *red as usize > *blue as usize + *green as usize {
                1
            } else {
                0
            }
        })
        .sum();
    info!("magical red count is: {result}");
    Ok(HttpResponse::Ok().body(result.to_string()))
}
