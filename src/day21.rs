use actix_web::{error, web, Scope};
use dms_coordinates::DMS;
use s2::{cell::Cell, cellid::CellID};
use tracing::info;

pub(crate) fn scope() -> Scope {
    web::scope("/21").route(
        "/coords/{binary}",
        web::get().to(task1_flat_squares_on_a_round_sphere),
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
