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
        resources::generation::get_generation,
        resources::growth_rate::get_growth_rate,
        resources::habitat::get_habitat,
        resources::language::get_language,
        resources::ping::get_ping,
        resources::pokemon::get_pokemon,
        resources::shape::get_shape,
        resources::species::get_species,
        resources::version::get_version,
        resources::version_group::get_version_group,
    ),
    tags(
        (name = "Misc", description = "Miscellaneous endpoints")
    ),
    components(
        schemas(
            generation::Generation,
            growth_rate::GrowthRate,
            language::Language,
            localized_values::LocalizedValues,
            localized_values::VersionedLocalizedValues,
            pokemon::Pokemon,
            pokemon_color::PokemonColor,
            pokemon_habitat::PokemonHabitat,
            pokemon_shape::PokemonShape,
            species::Species,
            version::Version,
            version_group::VersionGroup,
            ColorBulkResponse,
            GenerationBulkResponse,
            GrowthRateBulkResponse,
            HabitatBulkResponse,
            LanguageBulkResponse,
            MessageResponse,
            PokemonBulkResponse,
            ShapeBulkResponse,
            SpeciesBulkResponse,
            VersionBulkResponse,
            VersionGroupBulkResponse,
        )
    )
)]
pub struct ApiDoc;