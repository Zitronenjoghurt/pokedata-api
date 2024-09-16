use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonShapeNamesCSV {
    pub pokemon_shape_id: u32,
    pub local_language_id: u32,
    pub name: String,
}

impl CSVEntity for PokemonShapeNamesCSV {
    fn file_name() -> &'static str {
        "pokemon_shape_prose"
    }
}

impl HasLocalizedValues for PokemonShapeNamesCSV {
    fn id(&self) -> u32 {
        self.pokemon_shape_id
    }

    fn language_id(&self) -> u32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonShapeAwesomeNamesCSV {
    pub pokemon_shape_id: u32,
    pub local_language_id: u32,
    pub awesome_name: String,
}

impl CSVEntity for PokemonShapeAwesomeNamesCSV {
    fn file_name() -> &'static str {
        "pokemon_shape_prose"
    }
}

impl HasLocalizedValues for PokemonShapeAwesomeNamesCSV {
    fn id(&self) -> u32 {
        self.pokemon_shape_id
    }

    fn language_id(&self) -> u32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.awesome_name.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonShapeDescriptionsCSV {
    pub pokemon_shape_id: u32,
    pub local_language_id: u32,
    pub description: String,
}

impl CSVEntity for PokemonShapeDescriptionsCSV {
    fn file_name() -> &'static str {
        "pokemon_shape_prose"
    }
}

impl HasLocalizedValues for PokemonShapeDescriptionsCSV {
    fn id(&self) -> u32 {
        self.pokemon_shape_id
    }

    fn language_id(&self) -> u32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.description.clone()
    }
}