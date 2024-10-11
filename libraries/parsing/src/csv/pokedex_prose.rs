use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokedexNamesCSV {
    pub pokedex_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for PokedexNamesCSV {
    fn file_name() -> &'static str {
        "pokedex_prose"
    }
}

impl HasLocalizedValues for PokedexNamesCSV {
    fn id(&self) -> i32 {
        self.pokedex_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokedexDescriptionsCSV {
    pub pokedex_id: i32,
    pub local_language_id: i32,
    pub description: String,
}

impl CSVEntity for PokedexDescriptionsCSV {
    fn file_name() -> &'static str {
        "pokedex_prose"
    }
}

impl HasLocalizedValues for PokedexDescriptionsCSV {
    fn id(&self) -> i32 {
        self.pokedex_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.description.clone()
    }
}