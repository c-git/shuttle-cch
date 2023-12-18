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

#[derive(Debug, Deserialize)]
struct Input {
    recipe: Ingredients,
    pantry: Ingredients,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Ingredients {
    flour: usize,
    sugar: usize,
    butter: usize,
    #[serde(rename = "baking powder")]
    baking_powder: usize,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: usize,
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
    if input.recipe.flour > 0 {
        cookies = cookies.min(input.pantry.flour / input.recipe.flour);
    }
    if input.recipe.sugar > 0 {
        cookies = cookies.min(input.pantry.sugar / input.recipe.sugar);
    }
    if input.recipe.butter > 0 {
        cookies = cookies.min(input.pantry.butter / input.recipe.butter);
    }
    if input.recipe.baking_powder > 0 {
        cookies = cookies.min(input.pantry.baking_powder / input.recipe.baking_powder);
    }
    if input.recipe.chocolate_chips > 0 {
        cookies = cookies.min(input.pantry.chocolate_chips / input.recipe.chocolate_chips);
    }
    let mut pantry = input.pantry.clone();
    pantry.flour -= input.recipe.flour * cookies;
    pantry.sugar -= input.recipe.sugar * cookies;
    pantry.butter -= input.recipe.butter * cookies;
    pantry.baking_powder -= input.recipe.baking_powder * cookies;
    pantry.chocolate_chips -= input.recipe.chocolate_chips * cookies;
    let result = Output { cookies, pantry };
    info!("result = {result:?}");

    Ok(HttpResponse::Ok().json(result))
}
