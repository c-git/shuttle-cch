use std::{
    fs::create_dir,
    path::Path,
    sync::OnceLock,
    time::{Duration, Instant},
};

use actix_web::{error, web, Scope};
use dms_coordinates::DMS;
use s2::{cell::Cell, cellid::CellID};
use tokio::{fs::read_to_string, io::AsyncWriteExt, sync::Mutex, time::sleep};
use tracing::info;

const CACHE_DIR: &str = "cache-day21";

type WrappedClient = Mutex<reqwest::Client>;
type WrappedInstant = Mutex<Instant>;
type AppData = web::Data<(WrappedClient, WrappedInstant)>;

pub(crate) fn scope() -> Scope {
    static ONCE_LOCK: OnceLock<AppData> = OnceLock::new();
    let client = ONCE_LOCK.get_or_init(|| {
        AppData::new((
            WrappedClient::new(
                reqwest::Client::builder()
                    .user_agent("cch-sol-day22")
                    .build()
                    .unwrap(),
            ),
            WrappedInstant::new(Instant::now()),
        ))
    });
    web::scope("/21")
        .app_data(client.clone())
        .route(
            "/coords/{binary}",
            web::get().to(task1_flat_squares_on_a_round_sphere),
        )
        .route(
            "/country/{binary}",
            web::get().to(task2_turbo_fast_country_lookup),
        )
}

#[tracing::instrument]
async fn task1_flat_squares_on_a_round_sphere(
    binary: web::Path<String>,
) -> actix_web::Result<String> {
    // Based on https://github.com/AntoniosBarotsis/shuttle-cch23/blob/c363b98d8026544c242c680935406fede6449e61/src/days/day_21.rs#L16-L27
    let value = u64::from_str_radix(&binary, 2).map_err(error::ErrorBadRequest)?;
    let cell_id = CellID(value);
    let cell = Cell::from(cell_id);
    let center = cell.center();
    let latitude = DMS::from_decimal_degrees(center.latitude().deg(), true);
    let longitude = DMS::from_decimal_degrees(center.longitude().deg(), false);
    let result = format!("{} {}", rounded_output(latitude), rounded_output(longitude));
    info!(result);
    Ok(result)
}

/// Needed because the default output has all decimal places - "83°39'54.323941915848685''N"
/// But we need 3 - "83°39'54.324''N"
fn rounded_output(value: DMS) -> String {
    format!(
        "{}°{}'{:.3}''{}",
        value.degrees, value.minutes, value.seconds, value.bearing
    )
}
#[tracing::instrument]
async fn task2_turbo_fast_country_lookup(
    binary: web::Path<String>,
    app_data: AppData,
) -> actix_web::Result<String> {
    let value = u64::from_str_radix(&binary, 2).map_err(error::ErrorBadRequest)?;
    let cell_id = CellID(value);
    let cell = Cell::from(cell_id);
    let center = cell.center();

    let result = lookup_country(app_data, center)
        .await
        .map_err(error::ErrorInternalServerError)?;
    info!(result);
    Ok(result)
}

async fn lookup_country(app_data: AppData, center: s2::point::Point) -> anyhow::Result<String> {
    let latitude = center.latitude().deg().to_string();
    let longitude = center.longitude().deg().to_string();
    let cache_key = format!("{latitude},{longitude}");
    let _ = create_dir(CACHE_DIR); // Ignore result as it will usually fail
    let cache_path = Path::new(CACHE_DIR).join(&cache_key);
    if let Ok(cached_value) = read_to_string(&cache_path).await {
        // Answer already in cache
        info!("Cache Hit for {cache_key}");
        return Ok(cached_value);
    }
    info!("Cache Miss for {cache_key}");

    // See https://nominatim.org/release-docs/develop/api/Reverse/
    // See note on accept-language
    let url = format!(
        "https://nominatim.openstreetmap.org/reverse?format=json&accept-language=en-US,en&lat={latitude}&lon={longitude}"
    );

    let client_wrapper = &app_data.0;
    let instant_wrapper = &app_data.1;
    let mut last_use = instant_wrapper.lock().await;
    while last_use.elapsed().as_secs() < 1 {
        // Sleep to prevent hitting the API too fast as per TOS
        // https://operations.osmfoundation.org/policies/nominatim/
        info!("Waiting before making request to rate limit API Use");
        sleep(Duration::from_secs(1)).await;
    }
    *last_use = Instant::now();
    let client = client_wrapper.lock().await;
    let result = client
        .get(url)
        .send()
        .await?
        .json::<ResponseData>()
        .await?
        .address
        .country;

    // Save to cache for next time
    let mut f = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(cache_path)
        .await?;
    f.write_all(result.as_bytes()).await?;

    Ok(result)
}

#[derive(Debug, serde::Deserialize)]
struct ResponseData {
    address: Address,
}

#[derive(Debug, serde::Deserialize)]
struct Address {
    country: String,
}
