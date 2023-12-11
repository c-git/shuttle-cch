use actix_web::{web, Scope};
use tracing::info;

pub(crate) fn scope() -> Scope {
    web::scope("/1").route("/{num1}/{num2}", web::get().to(task1_cube_the_bits))
}

#[derive(Debug, serde::Deserialize)]
struct Nums {
    num1: i32,
    num2: i32,
}

#[tracing::instrument]
async fn task1_cube_the_bits(nums: web::Path<Nums>) -> actix_web::Result<String> {
    let result = format!("{}", (nums.num1 ^ nums.num2).pow(3));
    info!("{result}");
    Ok(result)
}
