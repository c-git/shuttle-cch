use actix_web::{error, web, Scope};
use anyhow::Context;
use tracing::info;

pub(crate) fn scope() -> Scope {
    web::scope("/1")
        .route("/{num1}/{num2}", web::get().to(task1_cube_the_bits))
        .route("/{args:.*}", web::get().to(task2_the_sled_id_system))
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

#[tracing::instrument]
async fn task2_the_sled_id_system(args: web::Path<String>) -> actix_web::Result<String> {
    let args: Vec<_> = args
        .split('/')
        .map(|x| {
            x.parse::<i32>()
                .with_context(|| format!("Failed to parse {x:?} as i32"))
                .map_err(error::ErrorBadRequest)
        })
        .collect();

    let mut numbers = Vec::with_capacity(args.len());
    for arg in args {
        numbers.push(arg?);
    }
    let mut result = numbers
        .into_iter()
        .reduce(|acc, x| acc ^ x)
        .expect("Shouldn't be empty, assumed to be 1 to 20 numbers");

    result = result.pow(3);
    info!("{result}");
    Ok(result.to_string())
}
