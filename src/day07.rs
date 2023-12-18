use std::collections::HashMap;

use actix_web::{error, web, HttpRequest, HttpResponse, Scope};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use tracing::info;
pub(crate) fn scope() -> Scope {
    web::scope("/7")
        .route("/decode", web::get().to(task1_based_encoding_64_th_edition))
        .route("/bake", web::get().to(task2_the_secret_cookie_recipe))
}

#[tracing::instrument]
async fn task1_based_encoding_64_th_edition(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let json = extract_recipe(req)?;
    Ok(HttpResponse::Ok().body(json))
}

#[tracing::instrument]
fn extract_recipe(req: HttpRequest) -> actix_web::Result<String> {
    let cookie = match req.cookie("recipe") {
        Some(val) => val,
        None => return Err(error::ErrorBadRequest("recipe cookie not found")),
    };
    info!("Incoming Cookie: {cookie}");

    let value = cookie.value();
    let decoded = general_purpose::STANDARD
        .decode(value)
        .map_err(error::ErrorBadRequest)?;
    let json = String::from_utf8(decoded).map_err(error::ErrorBadRequest)?;
    info!(json);
    Ok(json)
}

type Ingredients = HashMap<String, usize>;
#[derive(Debug, Deserialize)]
struct Input {
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(Debug, Serialize)]
struct Output {
    cookies: usize,
    pantry: Ingredients,
}

#[tracing::instrument]
async fn task2_the_secret_cookie_recipe(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let input: Input =
        serde_json::from_str(&extract_recipe(req)?).map_err(error::ErrorBadRequest)?;
    info!("Parsed Input = {input:?}");

    let mut cookies = usize::MAX;
    for (ingredient, needed) in input.recipe.iter() {
        if needed == &0 {
            // We don't need this just skip
            continue;
        }
        let available = if let Some(qty) = input.pantry.get(ingredient) {
            qty
        } else {
            cookies = 0;
            break;
        };
        cookies = cookies.min(available / needed);
    }

    let mut pantry = input.pantry.clone();
    for (ingredient, qty) in pantry.iter_mut() {
        if let Some(needed) = input.recipe.get(ingredient) {
            *qty -= needed * cookies;
        }
    }
    // pantry.flour -= input.recipe.flour * cookies;
    // pantry.sugar -= input.recipe.sugar * cookies;
    // pantry.butter -= input.recipe.butter * cookies;
    // pantry.baking_powder -= input.recipe.baking_powder * cookies;
    // pantry.chocolate_chips -= input.recipe.chocolate_chips * cookies;
    let result = Output { cookies, pantry };
    info!("result = {result:?}");

    Ok(HttpResponse::Ok().json(result))
}
