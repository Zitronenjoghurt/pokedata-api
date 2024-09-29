use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use crate::traits::has_version_id::HasVersionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesFlavorTextCSV {
    pub species_id: u32,
    pub version_id: u32,
    pub language_id: u32,
    pub flavor_text: String,
}

impl CSVEntity for PokemonSpeciesFlavorTextCSV {
    fn file_name() -> &'static str {
        "pokemon_species_flavor_text"
    }
}

impl HasVersionId for PokemonSpeciesFlavorTextCSV {
    fn version_id(&self) -> u32 {
        self.version_id
    }
}

impl HasLocalizedValues for PokemonSpeciesFlavorTextCSV {
    fn id(&self) -> u32 {
        self.species_id
    }

    fn language_id(&self) -> u32 {
        self.language_id
    }

    fn name(&self) -> String {
        self.flavor_text.clone()
    }
}