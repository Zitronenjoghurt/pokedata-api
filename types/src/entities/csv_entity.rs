use crate::entities::csv::abilities::AbilitiesCSV;
use crate::entities::csv::evolution_chains::EvolutionChainsCSV;
use crate::entities::csv::generation_names::GenerationNamesCSV;
use crate::entities::csv::generations::GenerationsCSV;
use crate::entities::csv::growth_rate_prose::GrowthRateProseCSV;
use crate::entities::csv::growth_rates::GrowthRatesCSV;
use crate::entities::csv::language_names::LanguageNamesCSV;
use crate::entities::csv::languages::LanguagesCSV;
use crate::entities::csv::move_target_prose::{MoveTargetDescriptionsCSV, MoveTargetNamesCSV};
use crate::entities::csv::move_targets::MoveTargetsCSV;
use crate::entities::csv::moves::MovesCSV;
use crate::entities::csv::pokemon::PokemonCSV;
use crate::entities::csv::pokemon_color_names::PokemonColorNamesCSV;
use crate::entities::csv::pokemon_colors::PokemonColorsCSV;
use crate::entities::csv::pokemon_evolution::PokemonEvolutionCSV;
use crate::entities::csv::pokemon_game_indices::PokemonVersionIdsCSV;
use crate::entities::csv::pokemon_habitat_names::PokemonHabitatNamesCSV;
use crate::entities::csv::pokemon_habitats::PokemonHabitatsCSV;
use crate::entities::csv::pokemon_shape_prose::{PokemonShapeAwesomeNamesCSV, PokemonShapeDescriptionsCSV, PokemonShapeNamesCSV};
use crate::entities::csv::pokemon_shapes::PokemonShapesCSV;
use crate::entities::csv::pokemon_species::PokemonSpeciesCSV;
use crate::entities::csv::pokemon_species_flavor_text::PokemonSpeciesFlavorTextCSV;
use crate::entities::csv::pokemon_species_names::PokemonSpeciesNamesCSV;
use crate::entities::csv::pokemon_stats::PokemonStatsCSV;
use crate::entities::csv::pokemon_types::PokemonTypesCSV;
use crate::entities::csv::pokemon_types_past::PokemonTypesPastCSV;
use crate::entities::csv::region_names::RegionNamesCSV;
use crate::entities::csv::regions::RegionsCSV;
use crate::entities::csv::stat_names::StatNamesCSV;
use crate::entities::csv::stats::StatsCSV;
use crate::entities::csv::type_efficacy::TypeEfficacyCSV;
use crate::entities::csv::type_efficacy_past::TypeEfficacyPastCSV;
use crate::entities::csv::type_names::TypeNamesCSV;
use crate::entities::csv::types::TypesCSV;
use crate::entities::csv::version_group_pokemon_move_methods::VersionGroupPokemonMoveMethodsCSV;
use crate::entities::csv::version_group_regions::VersionGroupRegionsCSV;
use crate::entities::csv::version_groups::VersionGroupsCSV;
use crate::entities::csv::version_names::VersionNamesCSV;
use crate::entities::csv::versions::VersionsCSV;
use csv::Reader;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct CSVEntityMetaData {
    pub file_name: String,
    pub download_url: String,
    pub is_downloadable: bool,
}

pub trait CSVEntity: DeserializeOwned {
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
        EvolutionChainsCSV::get_metadata(),
        GenerationNamesCSV::get_metadata(),
        GenerationsCSV::get_metadata(),
        GrowthRateProseCSV::get_metadata(),
        GrowthRatesCSV::get_metadata(),
        LanguagesCSV::get_metadata(),
        LanguageNamesCSV::get_metadata(),
        MovesCSV::get_metadata(),
        MoveTargetsCSV::get_metadata(),
        MoveTargetDescriptionsCSV::get_metadata(),
        MoveTargetNamesCSV::get_metadata(),
        PokemonCSV::get_metadata(),
        PokemonColorNamesCSV::get_metadata(),
        PokemonColorsCSV::get_metadata(),
        PokemonEvolutionCSV::get_metadata(),
        PokemonHabitatNamesCSV::get_metadata(),
        PokemonHabitatsCSV::get_metadata(),
        PokemonShapeAwesomeNamesCSV::get_metadata(),
        PokemonShapeDescriptionsCSV::get_metadata(),
        PokemonShapeNamesCSV::get_metadata(),
        PokemonShapesCSV::get_metadata(),
        PokemonSpeciesCSV::get_metadata(),
        PokemonSpeciesFlavorTextCSV::get_metadata(),
        PokemonSpeciesNamesCSV::get_metadata(),
        PokemonStatsCSV::get_metadata(),
        PokemonTypesCSV::get_metadata(),
        PokemonTypesPastCSV::get_metadata(),
        PokemonVersionIdsCSV::get_metadata(),
        RegionNamesCSV::get_metadata(),
        RegionsCSV::get_metadata(),
        StatNamesCSV::get_metadata(),
        StatsCSV::get_metadata(),
        TypeEfficacyCSV::get_metadata(),
        TypeEfficacyPastCSV::get_metadata(),
        TypeNamesCSV::get_metadata(),
        TypesCSV::get_metadata(),
        VersionGroupPokemonMoveMethodsCSV::get_metadata(),
        VersionGroupRegionsCSV::get_metadata(),
        VersionGroupsCSV::get_metadata(),
        VersionNamesCSV::get_metadata(),
        VersionsCSV::get_metadata(),
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