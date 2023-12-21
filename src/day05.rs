use actix_web::{error, web, HttpResponse, Scope};
use tracing::info;

pub(crate) fn scope() -> Scope {
    web::scope("/5").route("", web::post().to(slicing_the_loop))
}

#[derive(Debug, serde::Deserialize)]
struct QueryData {
    offset: usize,
    limit: usize,
}

#[tracing::instrument]
async fn slicing_the_loop(
    web::Json(names): web::Json<Vec<String>>,
    web::Query(QueryData { offset, limit }): web::Query<QueryData>,
) -> actix_web::Result<HttpResponse> {
    let result = match names.get(offset..(offset + limit)) {
        Some(x) => x,
        None => {
            return Err(error::ErrorBadRequest(format!(
                "Error: Out of bounds! names len: {} offset={offset} limit={limit}",
                names.len()
            )));
        }
    };
    info!("result={result:?}");
    Ok(HttpResponse::Ok().json(result))
}
