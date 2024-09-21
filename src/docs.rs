use crate::models::{bulk_response::*, message_response::MessageResponse, type_effectiveness::*};
use crate::resources;
use pokedata_api_types::entities::api::*;
use pokemon_color::*;
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
        resources::pokemon_type::get_pokemon_type,
        resources::region::get_region,
        resources::shape::get_shape,
        resources::stat::get_stat,
        resources::pokemon_type_efficacy::get_type_efficacy,
        resources::pokemon_type_efficacy::get_type_efficacy_all,
        resources::species::get_species,
        resources::version::get_version,
        resources::version_group::get_version_group,
    ),
    tags(
        (name = "Misc", description = "Miscellaneous endpoints"),
        (name = "Colors"),
        (name = "Generations"),
        (name = "Growth Rates"),
        (name = "Habitats"),
        (name = "Languages"),
        (name = "Pokemon"),
        (name = "Pokemon Stats"),
        (name = "Regions"),
        (name = "Shapes"),
        (name = "Species"),
        (name = "Types"),
        (name = "Versions"),
        (name = "Version Groups"),
    ),
    components(
        schemas(
            generation::Generation,
            growth_rate::GrowthRate,
            language::Language,
            localized_values::LocalizedValues,
            localized_values::VersionedLocalizedValues,
            localized_values::VersionGroupedLocalizedValues,
            pokemon::Pokemon,
            PokemonColor,
            pokemon_habitat::PokemonHabitat,
            pokemon_shape::PokemonShape,
            pokemon_type::PokemonType,
            pokemon_type_efficacy::PokemonTypeEfficacies,
            pokemon_type_efficacy::PokemonTypeEfficacyEntry,
            species::Species,
            version::Version,
            version_group::VersionGroup,
            AllTypeEffectivenessResponse,
            ColorBulkResponse,
            GenerationBulkResponse,
            GrowthRateBulkResponse,
            HabitatBulkResponse,
            LanguageBulkResponse,
            PokemonBulkResponse,
            PokemonTypeBulkResponse,
            RegionBulkResponse,
            ShapeBulkResponse,
            SpeciesBulkResponse,
            StatBulkResponse,
            VersionBulkResponse,
            VersionGroupBulkResponse,
            MessageResponse,
            TypeEffectivenessResponse,
        )
    )
)]
pub struct ApiDoc;