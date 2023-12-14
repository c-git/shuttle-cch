use std::{collections::HashMap, sync::OnceLock};

use actix_web::{web, HttpResponse, Scope};
use tokio::{sync::Mutex, time::Instant};
use tracing::info;

type Map = HashMap<String, Instant>;
type WrappedMap = Mutex<Map>;
type AppData = web::Data<WrappedMap>;

pub(crate) fn scope() -> Scope {
    // Decided to use OnceLock instead of creating MapOutside to keep days contained
    // Thus it should be noted this is not needed if map is created in main.rs
    static ONCE_LOCK: OnceLock<AppData> = OnceLock::new();
    if ONCE_LOCK.get().is_none() {
        ONCE_LOCK
            .set(AppData::new(WrappedMap::new(Map::new())))
            .expect("Just checked that it was empty");
    }
    let map = ONCE_LOCK.get().expect("Just ensured it was set");
    web::scope("/12")
        .app_data(map.clone())
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
