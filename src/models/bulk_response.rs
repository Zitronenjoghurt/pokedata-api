use pokedata_api_types::entities::api::language::Language;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LanguageBulkResponse {
    pub count: usize,
    pub results: Vec<Language>,
}