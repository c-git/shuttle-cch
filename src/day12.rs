use std::collections::HashMap;

use actix_web::{web, HttpResponse, Scope};
use tokio::{sync::Mutex, time::Instant};
use tracing::info;

type Map = HashMap<String, Instant>;
type WrappedMap = Mutex<Map>;

pub(crate) fn scope() -> Scope {
    let map = web::Data::new(WrappedMap::new(Map::new()));
    web::scope("/12")
        .app_data(map)
        .route("/save/{id}", web::post().to(task1_save))
        .route("/load/{id}", web::get().to(task1_load))
}

#[tracing::instrument]
async fn task1_save(id: web::Path<String>, map: web::Data<WrappedMap>) -> HttpResponse {
    let mut map = map.lock().await;
    let id = id.into_inner();
    info!("Saving ID: {id:?}");
    map.insert(id, Instant::now());
    HttpResponse::Ok().finish()
}

#[tracing::instrument]
async fn task1_load(
    id: web::Path<String>,
    map: web::Data<WrappedMap>,
) -> actix_web::Result<HttpResponse> {
    let result = 4;
    info!("Result = {result}");
    Ok(HttpResponse::Ok().body(result.to_string()))
}
