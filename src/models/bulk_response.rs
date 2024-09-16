use pokedata_api_types::entities::api::language::Language;
use pokedata_api_types::entities::api::species::Species;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LanguageBulkResponse {
    pub count: usize,
    pub results: Vec<Language>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SpeciesBulkResponse {
    pub count: usize,
    pub results: Vec<Species>,
}