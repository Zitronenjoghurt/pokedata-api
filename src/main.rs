use axum::Router;
use bincode;
use once_cell::sync::Lazy;
use pokedata_api_types::app_state::AppState;
use std::io;
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
static APP_STATE: Lazy<AppState> = Lazy::new(|| bincode::deserialize(STATIC_DATA).unwrap());

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, build_app()).await
}

pub fn get_app_state() -> &'static AppState {
    &APP_STATE
}

pub fn build_app() -> Router {
    Router::new()
        .nest("/", resources::ping::router())
        .nest("/color", resources::color::router())
        .nest("/evolution", resources::evolution::router())
        .nest("/evolution-chain", resources::evolution_chain::router())
        .nest("/generation", resources::generation::router())
        .nest("/growth-rate", resources::growth_rate::router())
        .nest("/habitat", resources::habitat::router())
        .nest("/language", resources::language::router())
        .nest("/pokemon", resources::pokemon::router())
        .nest("/pokemon-type", resources::pokemon_type::router())
        .nest("/region", resources::region::router())
        .nest("/shape", resources::shape::router())
        .nest("/species", resources::species::router())
        .nest("/stat", resources::stat::router())
        .nest("/type-efficacy", resources::pokemon_type_efficacy::router())
        .nest("/version", resources::version::router())
        .nest("/version-group", resources::version_group::router())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", docs::ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", docs::ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/docs"))
        .with_state(get_app_state().clone())
}