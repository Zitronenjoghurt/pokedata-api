use crate::models::{bulk_response::*, message_response::MessageResponse};
use crate::resources;
use pokedata_api_types::entities::api::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "PokeData API",
        description = "An API handling all kinds of pokemon data. Accumulated directly from the data sources of PokeAPI and PokemonTCG.io"
    ),
    paths(
        resources::color::get_color,
        resources::habitat::get_habitat,
        resources::language::get_language,
        resources::ping::get_ping,
        resources::shape::get_shape,
        resources::species::get_species,
    ),
    tags(
        (name = "Misc", description = "Miscellaneous endpoints")
    ),
    components(
        schemas(
            language::Language,
            localized_values::LocalizedValues,
            pokemon_color::PokemonColor,
            pokemon_habitat::PokemonHabitat,
            pokemon_shape::PokemonShape,
            species::Species,
            ColorBulkResponse,
            HabitatBulkResponse,
            LanguageBulkResponse,
            MessageResponse,
            ShapeBulkResponse,
            SpeciesBulkResponse
        )
    )
)]
pub struct ApiDoc;