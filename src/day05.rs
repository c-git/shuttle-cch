use actix_web::{error, web, HttpResponse, Scope};
use tracing::info;

pub(crate) fn scope() -> Scope {
    web::scope("/5").route("", web::post().to(slicing_the_loop))
}

#[derive(Debug, serde::Deserialize)]
struct QueryData {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

#[tracing::instrument]
async fn slicing_the_loop(
    web::Json(names): web::Json<Vec<String>>,
    web::Query(QueryData {
        offset,
        limit,
        split,
    }): web::Query<QueryData>,
) -> actix_web::Result<HttpResponse> {
    let offset = offset.unwrap_or_default();
    let remaining_size = names.len() - offset;
    let limit = match limit {
        Some(x) if x <= remaining_size => x,
        _ => remaining_size,
    };

    let slice = match names.get(offset..(offset + limit)) {
        Some(x) => x,
        None => {
            return Err(error::ErrorBadRequest(format!(
                "Error: Out of bounds! names len: {} offset={offset} limit={limit}",
                names.len()
            )));
        }
    };

    if let Some(x) = split {
        if x == 0 {
            return Err(error::ErrorBadRequest("split cannot be 0"));
        }
        let mut result = Vec::with_capacity(slice.len() / x + 1);
        for chunk in slice.chunks(x) {
            result.push(chunk)
        }
        info!("result={result:?}");
        Ok(HttpResponse::Ok().json(result))
    } else {
        info!("result={slice:?}");
        Ok(HttpResponse::Ok().json(slice))
    }
}
