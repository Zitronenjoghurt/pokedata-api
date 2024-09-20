use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::pokemon_type::get_major_type_ids;
use pokedata_api_types::entities::api::*;
use pokedata_api_types::entities::csv::abilities::AbilitiesCSV;
use pokedata_api_types::entities::csv::generations::GenerationsCSV;
use pokedata_api_types::entities::csv::growth_rates::GrowthRatesCSV;
use pokedata_api_types::entities::csv::languages::LanguagesCSV;
use pokedata_api_types::entities::csv::pokemon::PokemonCSV;
use pokedata_api_types::entities::csv::pokemon_colors::PokemonColorsCSV;
use pokedata_api_types::entities::csv::pokemon_habitats::PokemonHabitatsCSV;
use pokedata_api_types::entities::csv::pokemon_shapes::{PokemonShapesCSV, PokemonShapesConversionData};
use pokedata_api_types::entities::csv::type_names::TypeNamesCSV;
use pokedata_api_types::entities::csv::types::TypesCSV;
use pokedata_api_types::entities::csv::versions::VersionsCSV;
use pokedata_api_types::entities::csv::*;
use pokedata_api_types::entities::csv_entity::CSVEntity;
use pokedata_api_types::entities::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_types::entities::traits::into_id_map::IntoIdMap;
use pokedata_api_types::entities::traits::into_localized_values_map::IntoLocalizedValuesMap;
use std::path::PathBuf;

pub fn create_app_state(data_path: &PathBuf) -> AppState {
    let color_names = pokemon_color_names::PokemonColorNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let generation_names = generation_names::GenerationNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let growth_rate_names = growth_rate_prose::GrowthRateProseCSV::load(data_path).unwrap().into_localized_values_map();
    let habitat_names = pokemon_habitat_names::PokemonHabitatNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let language_names = language_names::LanguageNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let type_names = TypeNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let version_names = version_names::VersionNamesCSV::load(data_path).unwrap().into_localized_values_map();

    let pokemon_shapes_data = PokemonShapesConversionData::load(data_path);

    let pokemon_species_csv = pokemon_species::PokemonSpeciesCSV::load(data_path).unwrap();
    let type_efficacy_csv = type_efficacy::TypeEfficacyCSV::load(data_path).unwrap();
    let type_efficacy_past_csv = type_efficacy_past::TypeEfficacyPastCSV::load(data_path).unwrap();
    let version_groups_csv = version_groups::VersionGroupsCSV::load(data_path).unwrap();

    let abilities = AbilitiesCSV::load_and_convert(data_path, &()).unwrap().into_id_map();
    let colors = PokemonColorsCSV::load_and_convert(data_path, &color_names).unwrap().into_id_map();
    let generations = GenerationsCSV::load_and_convert(data_path, &generation_names).unwrap().into_id_map();
    let growth_rates = GrowthRatesCSV::load_and_convert(data_path, &growth_rate_names).unwrap().into_id_map();
    let habitats = PokemonHabitatsCSV::load_and_convert(data_path, &habitat_names).unwrap().into_id_map();
    let languages = LanguagesCSV::load_and_convert(data_path, &language_names).unwrap().into_id_map();
    let pokemon = PokemonCSV::load_and_convert(data_path, &()).unwrap().into_id_map();
    let shapes = PokemonShapesCSV::load_and_convert(data_path, &pokemon_shapes_data).unwrap().into_id_map();
    let types = TypesCSV::load_and_convert(data_path, &type_names).unwrap().into_id_map();
    let versions = VersionsCSV::load_and_convert(data_path, &version_names).unwrap().into_id_map();

    let latest_generation = generations.keys().max().copied().unwrap_or(1);
    let major_type_ids = get_major_type_ids(types.values().cloned().collect());

    let type_efficacies = pokemon_type_efficacy::build_efficacies_by_generation(
        type_efficacy_csv,
        type_efficacy_past_csv,
        latest_generation,
    );

    let version_groups = version_group::build_version_groups(
        version_groups_csv,
        versions.clone(),
        data_path,
    ).into_id_map();

    let species = species::build_species(
        pokemon_species_csv,
        pokemon.clone(),
        data_path,
    ).into_id_map();

    AppState {
        abilities,
        colors,
        generations,
        growth_rates,
        habitats,
        languages,
        pokemon,
        shapes,
        species,
        types,
        type_efficacies,
        versions,
        version_groups,
        latest_generation,
        major_type_ids,
    }
}