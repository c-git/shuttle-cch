use std::sync::OnceLock;

use actix_web::{error, web, Scope};
use dms_coordinates::DMS;
use s2::{cell::Cell, cellid::CellID};
use tokio::sync::Mutex;
use tracing::info;

type WrappedClient = Mutex<reqwest::Client>;
type AppData = web::Data<WrappedClient>;

pub(crate) fn scope() -> Scope {
    static ONCE_LOCK: OnceLock<AppData> = OnceLock::new();
    let client = ONCE_LOCK.get_or_init(|| {
        AppData::new(WrappedClient::new(
            reqwest::Client::builder()
                .user_agent("cch-sol-day22")
                .build()
                .unwrap(),
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
    client_wrapper: AppData,
) -> actix_web::Result<String> {
    let value = u64::from_str_radix(&binary, 2).map_err(error::ErrorBadRequest)?;
    let cell_id = CellID(value);
    let cell = Cell::from(cell_id);
    let center = cell.center();

    // See https://nominatim.org/release-docs/develop/api/Reverse/
    // See note on accept-language
    let url = format!(
        "https://nominatim.openstreetmap.org/reverse?format=json&accept-language=en-US,en&lat={}&lon={}",
        center.latitude().deg(),
        center.longitude().deg()
    );

    let client = client_wrapper.lock().await;
    let result = client
        .get(url)
        .send()
        .await
        .map_err(error::ErrorInternalServerError)?
        .json::<ResponseData>()
        .await
        .map_err(error::ErrorInternalServerError)?
        .address
        .country;

    info!(result);
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
