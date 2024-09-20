use generation::*;
use growth_rate::*;
use language::*;
use pokedata_api_types::entities::api::*;
use pokemon::*;
use pokemon_color::*;
use pokemon_habitat::*;
use pokemon_shape::*;
use pokemon_type::*;
use serde::{Deserialize, Serialize};
use species::*;
use utoipa::ToSchema;
use version::*;
use version_group::*;

#[derive(Serialize, Deserialize, ToSchema)]
#[aliases(
    ColorBulkResponse = BulkResponse<PokemonColor>,
    GenerationBulkResponse = BulkResponse<Generation>,
    GrowthRateBulkResponse = BulkResponse<GrowthRate>,
    HabitatBulkResponse = BulkResponse<PokemonHabitat>,
    LanguageBulkResponse = BulkResponse<Language>,
    PokemonBulkResponse = BulkResponse<Pokemon>,
    PokemonTypeBulkResponse = BulkResponse<PokemonType>,
    ShapeBulkResponse = BulkResponse<PokemonShape>,
    SpeciesBulkResponse = BulkResponse<Species>,
    VersionBulkResponse = BulkResponse<Version>,
    VersionGroupBulkResponse = BulkResponse<VersionGroup>
)]
pub struct BulkResponse<T: for<'s> ToSchema<'s>>
{
    pub count: usize,
    pub results: Vec<T>,
}