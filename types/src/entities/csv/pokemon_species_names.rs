use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesNamesCSV {
    pub pokemon_species_id: u32,
    pub local_language_id: u32,
    pub name: String,
}

impl CSVEntity for PokemonSpeciesNamesCSV {
    fn file_name() -> &'static str {
        "pokemon_species_names"
    }
}

impl HasLocalizedValues for PokemonSpeciesNamesCSV {
    fn id(&self) -> u32 {
        self.pokemon_species_id
    }

    fn language_id(&self) -> u32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesGeneraCSV {
    pub pokemon_species_id: u32,
    pub local_language_id: u32,
    pub genus: Option<String>,
}

impl CSVEntity for PokemonSpeciesGeneraCSV {
    fn file_name() -> &'static str {
        "pokemon_species_names"
    }
}

impl HasLocalizedValues for PokemonSpeciesGeneraCSV {
    fn id(&self) -> u32 {
        self.pokemon_species_id
    }

    fn language_id(&self) -> u32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.genus.clone().unwrap_or_else(|| "not translated".to_string())
    }
}