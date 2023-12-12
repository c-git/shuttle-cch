use actix_files::NamedFile;
use actix_web::{web, Scope};

pub(crate) fn scope() -> Scope {
    web::scope("/11").route(
        "/assets/decoration.png",
        web::get().to(task1_served_on_a_silver_platter),
    )
}

#[tracing::instrument]
async fn task1_served_on_a_silver_platter() -> actix_web::Result<NamedFile> {
    NamedFile::open_async("assets/decoration.png")
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
}
