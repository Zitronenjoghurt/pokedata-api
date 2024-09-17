use pokedata_api_types::entities::api::generation::Generation;
use pokedata_api_types::entities::api::growth_rate::GrowthRate;
use pokedata_api_types::entities::api::language::Language;
use pokedata_api_types::entities::api::pokemon::Pokemon;
use pokedata_api_types::entities::api::pokemon_color::PokemonColor;
use pokedata_api_types::entities::api::pokemon_habitat::PokemonHabitat;
use pokedata_api_types::entities::api::pokemon_shape::PokemonShape;
use pokedata_api_types::entities::api::species::Species;
use pokedata_api_types::entities::api::version::Version;
use pokedata_api_types::entities::api::version_group::VersionGroup;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ColorBulkResponse {
    pub count: usize,
    pub results: Vec<PokemonColor>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GenerationBulkResponse {
    pub count: usize,
    pub results: Vec<Generation>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GrowthRateBulkResponse {
    pub count: usize,
    pub results: Vec<GrowthRate>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HabitatBulkResponse {
    pub count: usize,
    pub results: Vec<PokemonHabitat>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LanguageBulkResponse {
    pub count: usize,
    pub results: Vec<Language>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PokemonBulkResponse {
    pub count: usize,
    pub results: Vec<Pokemon>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ShapeBulkResponse {
    pub count: usize,
    pub results: Vec<PokemonShape>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SpeciesBulkResponse {
    pub count: usize,
    pub results: Vec<Species>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct VersionBulkResponse {
    pub count: usize,
    pub results: Vec<Version>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct VersionGroupBulkResponse {
    pub count: usize,
    pub results: Vec<VersionGroup>,
}