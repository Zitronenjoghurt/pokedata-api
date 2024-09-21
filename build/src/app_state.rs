use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::pokemon_type::get_major_type_ids;
use pokedata_api_types::entities::csv::pokemon_shapes::PokemonShapesConversionData;
use pokedata_api_types::entities::csv::type_efficacy::build_efficacies_by_generation;
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
    let region_names = region_names::RegionNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let stat_names = stat_names::StatNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let type_names = type_names::TypeNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let version_names = version_names::VersionNamesCSV::load(data_path).unwrap().into_localized_values_map();

    let pokemon_shapes_data = PokemonShapesConversionData::load(data_path);

    let abilities = abilities::AbilitiesCSV::load_and_convert(data_path, &()).unwrap().into_id_map();
    let colors = pokemon_colors::PokemonColorsCSV::load_and_convert(data_path, &color_names).unwrap().into_id_map();
    let generations = generations::GenerationsCSV::load_and_convert(data_path, &generation_names).unwrap().into_id_map();
    let growth_rates = growth_rates::GrowthRatesCSV::load_and_convert(data_path, &growth_rate_names).unwrap().into_id_map();
    let habitats = pokemon_habitats::PokemonHabitatsCSV::load_and_convert(data_path, &habitat_names).unwrap().into_id_map();
    let languages = languages::LanguagesCSV::load_and_convert(data_path, &language_names).unwrap().into_id_map();
    let pokemon = pokemon::PokemonCSV::load_and_convert(data_path, &()).unwrap().into_id_map();
    let regions = regions::RegionsCSV::load_and_convert(data_path, &region_names).unwrap().into_id_map();
    let shapes = pokemon_shapes::PokemonShapesCSV::load_and_convert(data_path, &pokemon_shapes_data).unwrap().into_id_map();
    let stats = stats::StatsCSV::load_and_convert(data_path, &stat_names).unwrap().into_id_map();
    let types = types::TypesCSV::load_and_convert(data_path, &type_names).unwrap().into_id_map();
    let versions = versions::VersionsCSV::load_and_convert(data_path, &version_names).unwrap().into_id_map();

    let latest_generation = generations.keys().max().copied().unwrap_or(1);
    let version_group_data = version_groups::VersionGroupConversionData::load(data_path, &versions);
    let species_data = pokemon_species::PokemonSpeciesConversionData::load(data_path, &pokemon);

    let species = pokemon_species::PokemonSpeciesCSV::load_and_convert(data_path, &species_data).unwrap().into_id_map();
    let type_efficacies = build_efficacies_by_generation(data_path, latest_generation).unwrap();
    let version_groups = version_groups::VersionGroupsCSV::load_and_convert(data_path, &version_group_data).unwrap().into_id_map();

    let major_type_ids = get_major_type_ids(types.values().cloned().collect());

    AppState {
        abilities,
        colors,
        generations,
        growth_rates,
        habitats,
        languages,
        pokemon,
        regions,
        shapes,
        species,
        stats,
        types,
        type_efficacies,
        versions,
        version_groups,
        latest_generation,
        major_type_ids,
    }
}