use crate::entities::csv::abilities::AbilitiesCSV;
use crate::entities::csv::generation_names::GenerationNamesCSV;
use crate::entities::csv::generations::GenerationsCSV;
use crate::entities::csv::growth_rate_prose::GrowthRateProseCSV;
use crate::entities::csv::growth_rates::GrowthRatesCSV;
use crate::entities::csv::language_names::LanguageNamesCSV;
use crate::entities::csv::languages::LanguagesCSV;
use crate::entities::csv::pokemon::PokemonCSV;
use crate::entities::csv::pokemon_color_names::PokemonColorNamesCSV;
use crate::entities::csv::pokemon_colors::PokemonColorsCSV;
use crate::entities::csv::pokemon_habitat_names::PokemonHabitatNamesCSV;
use crate::entities::csv::pokemon_habitats::PokemonHabitatsCSV;
use crate::entities::csv::pokemon_shape_prose::{PokemonShapeAwesomeNamesCSV, PokemonShapeDescriptionsCSV, PokemonShapeNamesCSV};
use crate::entities::csv::pokemon_shapes::PokemonShapesCSV;
use crate::entities::csv::pokemon_species::PokemonSpeciesCSV;
use crate::entities::csv::pokemon_species_flavor_text::PokemonSpeciesFlavorTextCSV;
use crate::entities::csv::pokemon_species_names::PokemonSpeciesNamesCSV;
use csv::Reader;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct CSVEntityMetaData {
    file_name: String,
    download_url: String,
    is_downloadable: bool,
}

pub trait CSVEntity {
    fn file_name() -> &'static str;

    fn base_download_url() -> &'static str {
        "https://raw.githubusercontent.com/PokeAPI/pokeapi/master/data/v2/csv/"
    }

    fn is_downloadable() -> bool {
        true
    }

    fn download_url() -> String {
        format!("{}{}.csv", Self::base_download_url(), Self::file_name())
    }

    fn load(base_path: &PathBuf) -> csv::Result<Vec<Self>>
    where
        Self: for<'de> serde::Deserialize<'de>,
    {
        let mut file_path = base_path.join(Self::file_name());
        file_path.set_extension("csv");
        let mut reader = Reader::from_path(file_path)?;
        reader.deserialize().collect()
    }

    fn get_metadata() -> CSVEntityMetaData {
        CSVEntityMetaData {
            file_name: Self::file_name().to_string(),
            download_url: Self::download_url(),
            is_downloadable: Self::is_downloadable(),
        }
    }
}

pub fn get_all_metadata() -> Vec<CSVEntityMetaData> {
    vec![
        AbilitiesCSV::get_metadata(),
        GenerationNamesCSV::get_metadata(),
        GenerationsCSV::get_metadata(),
        GrowthRateProseCSV::get_metadata(),
        GrowthRatesCSV::get_metadata(),
        LanguagesCSV::get_metadata(),
        LanguageNamesCSV::get_metadata(),
        PokemonCSV::get_metadata(),
        PokemonColorNamesCSV::get_metadata(),
        PokemonColorsCSV::get_metadata(),
        PokemonHabitatNamesCSV::get_metadata(),
        PokemonHabitatsCSV::get_metadata(),
        PokemonShapeAwesomeNamesCSV::get_metadata(),
        PokemonShapeDescriptionsCSV::get_metadata(),
        PokemonShapeNamesCSV::get_metadata(),
        PokemonShapesCSV::get_metadata(),
        PokemonSpeciesCSV::get_metadata(),
        PokemonSpeciesFlavorTextCSV::get_metadata(),
        PokemonSpeciesNamesCSV::get_metadata(),
    ]
}

/// Generates a map where the keys are CSV file names and the values are the download URLs
pub fn get_download_map() -> HashMap<String, String> {
    get_all_metadata()
        .into_iter()
        .filter(|metadata| metadata.is_downloadable)
        .map(|metadata| (metadata.file_name, metadata.download_url))
        .collect()
}