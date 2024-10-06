use axum::Router;
use once_cell::sync::Lazy;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;
use tokio::io;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod docs;
pub mod models;
pub mod queries;
pub mod resources;

#[cfg(test)]
pub mod tests;

pub mod serde {
    pub mod comma_seperated;
}

const STATIC_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/data.bin"));
static APP_STATE: Lazy<Arc<AppState>> = Lazy::new(||
    Arc::new(bincode::deserialize(STATIC_DATA).unwrap())
);

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, build_app()).await
}

pub fn get_app_state() -> Arc<AppState> {
    APP_STATE.clone()
}

pub fn build_app() -> Router {
    let app_state = get_app_state();

    Router::new()
        .nest("/", resources::ping::router())
        .nest("/ability", resources::ability::router())
        .nest("/berry", resources::berry::router())
        .nest("/berry-firmness", resources::berry_firmness::router())
        .nest("/berry-flavor", resources::berry_flavor::router())
        .nest("/color", resources::color::router())
        .nest("/egg-group", resources::egg_group::router())
        .nest("/evolution", resources::evolution::router())
        .nest("/evolution-chain", resources::evolution_chain::router())
        .nest("/generation", resources::generation::router())
        .nest("/growth-rate", resources::growth_rate::router())
        .nest("/habitat", resources::habitat::router())
        .nest("/language", resources::language::router())
        .nest("/move", resources::pokemon_move::router())
        .nest("/move-ailment", resources::pokemon_move_ailment::router())
        .nest("/move-category", resources::pokemon_move_category::router())
        .nest("/move-damage-class", resources::pokemon_move_damage_class::router())
        .nest("/move-effect", resources::pokemon_move_effect::router())
        .nest("/move-flag", resources::pokemon_move_flag::router())
        .nest("/move-target", resources::pokemon_move_target::router())
        .nest("/pokemon", resources::pokemon::router())
        .nest("/pokemon-type", resources::pokemon_type::router())
        .nest("/region", resources::region::router())
        .nest("/shape", resources::shape::router())
        .nest("/species", resources::species::router())
        .nest("/stat", resources::stat::router())
        .nest("/tcg-card", resources::tcg_card::router())
        .nest("/tcg-set", resources::tcg_set::router())
        .nest("/type-efficacy", resources::pokemon_type_efficacy::router())
        .nest("/version", resources::version::router())
        .nest("/version-group", resources::version_group::router())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", docs::ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", docs::ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/docs"))
        .with_state(app_state)
}