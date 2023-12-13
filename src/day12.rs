use actix_web::{web, Scope};
use tracing::info;

pub(crate) fn scope() -> Scope {
    web::scope("/12")
        .route("/save/{id}", web::post().to(task1_save))
        .route("/load/{id}", web::get().to(task1_load))
}

#[tracing::instrument]
async fn task1_save() -> String {
    info!("Task 1 save");
    todo!()
}

#[tracing::instrument]
async fn task1_load() -> String {
    info!("Task 1 load");
    todo!()
}
