use crate::entities::api::localized_values::LocalizedValues;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonShape {
    pub id: u32,
    pub identifier: String,
    pub names: Option<LocalizedValues>,
    /// The "scientific" name of this Pokémon shape listed in different languages.
    pub awesome_names: Option<LocalizedValues>,
    pub descriptions: Option<LocalizedValues>,
}

impl HasId for PokemonShape {
    fn id(&self) -> u32 {
        self.id
    }
}