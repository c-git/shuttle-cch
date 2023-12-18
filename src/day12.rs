use std::{collections::HashMap, sync::OnceLock};

use actix_web::{
    error,
    web::{self, Json},
    HttpResponse, Scope,
};
use chrono::{Datelike, Local};
use serde::Serialize;
use tokio::{sync::Mutex, time::Instant};
use tracing::info;
use ulid_generator_rs::{Endian, ULID};
use uuid::Uuid;

type Map = HashMap<String, Instant>;
type WrappedMap = Mutex<Map>;
type AppData = web::Data<WrappedMap>;

pub(crate) fn scope() -> Scope {
    // Decided to use OnceLock instead of creating the map outside to keep days contained
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
        .route(
            "/ulids",
            web::post().to(task2_unanimously_legendary_identifier_ulid),
        )
        .route(
            "/ulids/{weekday}",
            web::post().to(task3_let_santa_broil_lsb),
        )
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
    let map = map.lock().await;
    let saved_instant = match map.get(id.as_ref()) {
        Some(value) => value,
        None => return Err(error::ErrorBadRequest("This ID has not been seen before")),
    };
    let result = Instant::now().duration_since(*saved_instant).as_secs();
    info!("Result = {result}");
    Ok(HttpResponse::Ok().body(result.to_string()))
}

#[tracing::instrument]
async fn task2_unanimously_legendary_identifier_ulid(
    Json(ulids): web::Json<Vec<ULID>>,
) -> actix_web::Result<HttpResponse> {
    let result: Vec<String> = ulids
        .into_iter()
        .rev()
        .map(|x| {
            let b_vec = x.to_byte_array(Endian::BE);
            let b_array = b_vec.try_into().map_err(|e: Vec<_>| {
                error::ErrorInternalServerError(format!(
                    "Failed to convert vec of length: {} into [u8; 16]",
                    e.len()
                ))
            })?;
            let uuid = Uuid::from_bytes(b_array);
            Ok(uuid.to_string())
        })
        .collect::<actix_web::Result<Vec<String>>>()?;
    info!("Result = {result:?}");
    Ok(HttpResponse::Ok().json(result))
}

#[derive(Serialize, Debug)]
struct Stats {
    #[serde(rename = "christmas eve")]
    on_christmas_eve: u16,
    weekday: u16,
    #[serde(rename = "in the future")]
    in_future: u16,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: u16,
}

#[tracing::instrument]
async fn task3_let_santa_broil_lsb(
    weekday: web::Path<u32>,
    Json(ulids): web::Json<Vec<ULID>>,
) -> actix_web::Result<HttpResponse> {
    let now = Local::now();
    let weekday = weekday.into_inner();

    let mut result = Stats {
        on_christmas_eve: 0,
        weekday: 0,
        in_future: 0,
        lsb_is_1: 0,
    };

    for ulid in ulids {
        let date_time = ulid.to_date_time().with_timezone(&chrono::Utc);
        if date_time.month() == 12 && date_time.day() == 24 {
            result.on_christmas_eve += 1;
        }
        if date_time.weekday().num_days_from_monday() == weekday {
            result.weekday += 1;
        }
        if date_time > now {
            result.in_future += 1;
        }
        if ulid.least_significant_bits() % 2 == 1 {
            result.lsb_is_1 += 1;
        }
    }
    info!("Result = {result:?}");
    Ok(HttpResponse::Ok().json(result))
}
