use crate::api::localized_values::LocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonType {
    pub id: i32,
    pub identifier: String,
    pub generation_id: i32,
    pub is_major_type: bool,
    pub damage_class_id: Option<i32>,
    pub names: Option<LocalizedValues>,
}

impl HasId for PokemonType {
    fn id(&self) -> i32 {
        self.id
    }
}

pub fn get_major_type_ids(types: Vec<PokemonType>) -> Vec<i32> {
    types.into_iter()
        .filter(|pokemon_type| pokemon_type.is_major_type)
        .map(|pokemon_type| pokemon_type.id)
        .collect()
}