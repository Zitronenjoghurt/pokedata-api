use crate::entities::api::localized_values::LocalizedValues;
use crate::entities::csv::type_names::TypeNamesCSV;
use crate::entities::csv::types::TypesCSV;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::has_id::HasId;
use crate::entities::traits::into_localized_values_map::IntoLocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
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

pub fn build_types(
    types_csv: Vec<TypesCSV>,
    data_path: &PathBuf,
) -> Vec<PokemonType> {
    let names_map = TypeNamesCSV::load(data_path).unwrap().into_localized_values_map();

    let mut types = Vec::with_capacity(types_csv.len());

    for entry in types_csv {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let identifier = match entry.identifier {
            Some(identifier) => identifier,
            None => continue,
        };

        let generation_id = match entry.generation_id {
            Some(generation_id) => generation_id,
            None => continue,
        };

        let pokemon_type = PokemonType {
            id,
            identifier,
            generation_id,
            is_major_type: entry.is_major_type.unwrap_or(0) == 1,
            damage_class_id: entry.damage_class_id,
            names: names_map.get(id),
        };

        types.push(pokemon_type);
    }

    types
}