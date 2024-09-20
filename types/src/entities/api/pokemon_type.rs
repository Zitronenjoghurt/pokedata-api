use crate::entities::api::localized_values::LocalizedValues;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonType {
    pub id: u32,
    pub identifier: String,
    pub generation_id: u32,
    pub is_major_type: bool,
    pub damage_class_id: Option<u32>,
    pub names: Option<LocalizedValues>,
}

impl HasId for PokemonType {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn get_major_type_ids(types: Vec<PokemonType>) -> Vec<u32> {
    types.into_iter()
        .filter(|pokemon_type| pokemon_type.is_major_type)
        .map(|pokemon_type| pokemon_type.id)
        .collect()
}