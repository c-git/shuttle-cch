use std::fs::File;

use actix_web::{
    web::{self},
    HttpResponse, Scope,
};
use async_tempfile::TempFile;
use futures_util::StreamExt;
use tar::Archive;
use tokio::io::AsyncWriteExt;

pub(crate) fn scope() -> Scope {
    web::scope("/20")
        .route(
            "/archive_files",
            web::post().to(task1_archive_analysis_count),
        )
        .route(
            "/archive_files_size",
            web::post().to(task1_archive_analysis_size),
        )
}

#[tracing::instrument(skip(body))]
async fn task1_archive_analysis_count(body: web::Payload) -> actix_web::Result<HttpResponse> {
    let f = receive_tar(body).await?;
    let g = File::open(f.file_path())?;
    let mut a = Archive::new(g);
    let result = a.entries()?.count();
    Ok(HttpResponse::Ok().body(result.to_string()))
}

#[tracing::instrument(skip(body))]
async fn task1_archive_analysis_size(body: web::Payload) -> actix_web::Result<HttpResponse> {
    let f = receive_tar(body).await?;
    let g = File::open(f.file_path())?;
    let mut a = Archive::new(g);
    let result: u64 = a.entries()?.map(|x| x.unwrap().size()).sum();
    Ok(HttpResponse::Ok().body(result.to_string()))
}

async fn receive_tar(mut body: web::Payload) -> actix_web::Result<TempFile> {
    let mut result = TempFile::new().await.unwrap();
    while let Some(item) = body.next().await {
        result.write_all(&item?).await?;
    }
    result.sync_all().await?;
    Ok(result)
}
