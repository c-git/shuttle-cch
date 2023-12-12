use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, Scope};
use futures_util::StreamExt as _;

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

#[tracing::instrument(skip(payload))]
async fn task2_bull_mode_activated(
    mut payload: Multipart,
    req: HttpRequest,
) -> actix_web::Result<NamedFile> {
    // From https://www.youtube.com/watch?v=NWxs6BQOzxU
    // iterate over multipart stream
    // while let Some(item) = payload.next().await {
    //     let mut field = item?;

    //     // Field in turn is stream of *Bytes* object
    //     while let Some(chunk) = field.next().await {
    //         println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk?));
    //     }
    // }
    todo!("Task2")
}
