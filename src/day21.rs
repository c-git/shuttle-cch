use actix_web::{error, web, Scope};
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
    let value = u64::from_str_radix(&binary, 2).map_err(error::ErrorBadRequest)?;
    info!(value);
    let my_value = convert_to_u64(&binary).map_err(error::ErrorBadRequest)?;
    if my_value == value {
        Ok("Yeah they match\n".to_string())
    } else {
        Err(error::ErrorInternalServerError(format!(
            "So they don't match my_value={my_value} vs value={value}\n"
        )))
    }
}

fn convert_to_u64(binary: &str) -> Result<u64, &'static str> {
    let mut result = 0;
    let bits = binary.as_bytes();
    if bits.len() > 64 {
        return Err("number to large to fit target type");
    }
    for bit in bits {
        result <<= 1;
        match bit {
            b'0' => {}
            b'1' => result += 1,
            _ => return Err("invalid digit found in string"),
        }
    }
    Ok(result)
}
