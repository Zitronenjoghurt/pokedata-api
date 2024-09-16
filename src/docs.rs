use crate::models::{bulk_response::LanguageBulkResponse, message_response::MessageResponse};
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
        resources::ping::get_ping,
        resources::language::get_language
    ),
    tags(
        (name = "Misc", description = "Miscellaneous endpoints")
    ),
    components(
        schemas(
            language::Language,
            localized_names::LocalizedNames,
            LanguageBulkResponse,
            MessageResponse
        )
    )
)]
pub struct ApiDoc;