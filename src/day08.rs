use actix_web::{error, web, Scope};
use anyhow::Context;
use rustemon::{client::RustemonClient, pokemon::pokemon};
use tracing::info;

pub(crate) fn scope() -> Scope {
    web::scope("/8").route("/weight/{pokedex_number}", web::get().to(task1_its_pikachu))
}

#[tracing::instrument]
async fn task1_its_pikachu(pokedex_number: web::Path<i64>) -> actix_web::Result<String> {
    let client: RustemonClient = Default::default();
    let weight = pokemon::get_by_id(pokedex_number.into_inner(), &client)
        .await
        .context("failed to get pokemon info")
        .map_err(error::ErrorBadRequest)?
        .weight
        / 10;
    info!(weight);

    Ok(weight.to_string())
}
