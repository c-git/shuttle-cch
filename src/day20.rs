use actix_web::{
    web::{self},
    HttpResponse, Scope,
};
use async_tempfile::TempFile;
use futures_util::StreamExt;
use tar::Archive;
use tokio::io::AsyncWriteExt;

pub(crate) fn scope() -> Scope {
    web::scope("/20").route(
        "/archive_files",
        web::post().to(task1_archive_analysis_count),
    )
    // .route(
    //     "/archive_files_size",
    //     web::post().to(task1_archive_analysis_size),
    // )
}

#[tracing::instrument(skip(body))]
async fn task1_archive_analysis_count(body: web::Payload) -> actix_web::Result<HttpResponse> {
    let a = receive_tar(body).await?;
    let b = a.metadata().await?;
    let c = b.len();

    // for file in a.entries().unwrap() {
    //     // Make sure there wasn't an I/O error
    //     let mut file = file.unwrap();

    //     // Inspect metadata about the file
    //     println!("{:?}", file.header().path().unwrap());
    //     println!("{}", file.header().size().unwrap());

    //     // files implement the Read trait
    //     let mut s = String::new();
    //     file.read_to_string(&mut s).unwrap();
    //     println!("{}", s);
    // }
    Ok(HttpResponse::Ok().body(c.to_string()))
}

#[tracing::instrument(skip(body))]
async fn task1_archive_analysis_size(body: web::Payload) -> actix_web::Result<HttpResponse> {
    let a = receive_tar(body).await?;
    todo!()
}

async fn receive_tar(mut body: web::Payload) -> actix_web::Result<TempFile> {
    let mut result = TempFile::new().await.unwrap();
    while let Some(item) = body.next().await {
        result.write_all(&item?).await?;
    }
    Ok(result)
}
