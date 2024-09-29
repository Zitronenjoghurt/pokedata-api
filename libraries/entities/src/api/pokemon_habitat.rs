use crate::api::localized_values::LocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonHabitat {
    pub id: u32,
    pub identifier: String,
    pub names: Option<LocalizedValues>,
}

impl HasId for PokemonHabitat {
    fn id(&self) -> u32 {
        self.id
    }
}