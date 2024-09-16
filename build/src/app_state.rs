use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::pokemon::build_pokemon;
use pokedata_api_types::entities::api::*;
use pokedata_api_types::entities::csv::pokemon::PokemonCSV;
use pokedata_api_types::entities::csv::*;
use pokedata_api_types::entities::csv_entity::CSVEntity;
use pokedata_api_types::entities::traits::into_id_map::IntoIdMap;
use pokedata_api_types::entities::traits::into_localized_values_map::IntoLocalizedValuesMap;
use std::path::PathBuf;

pub fn create_app_state(data_path: &PathBuf) -> AppState {
    let abilities_csv = abilities::AbilitiesCSV::load(data_path).unwrap();
    let generations_csv = generations::GenerationsCSV::load(data_path).unwrap();
    let growth_rates_csv = growth_rates::GrowthRatesCSV::load(data_path).unwrap();
    let languages_csv = languages::LanguagesCSV::load(data_path).unwrap();
    let pokemon_csv = PokemonCSV::load(data_path).unwrap();
    let pokemon_colors_csv = pokemon_colors::PokemonColorsCSV::load(data_path).unwrap();
    let pokemon_habitats_csv = pokemon_habitats::PokemonHabitatsCSV::load(data_path).unwrap();
    let pokemon_shapes_csv = pokemon_shapes::PokemonShapesCSV::load(data_path).unwrap();
    let pokemon_species_csv = pokemon_species::PokemonSpeciesCSV::load(data_path).unwrap();

    let color_names_map = pokemon_color_names::PokemonColorNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let generation_names_map = generation_names::GenerationNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let growth_rate_names_map = growth_rate_prose::GrowthRateProseCSV::load(data_path).unwrap().into_localized_values_map();
    let habitat_names_map = pokemon_habitat_names::PokemonHabitatNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let language_names_map = language_names::LanguageNamesCSV::load(data_path).unwrap().into_localized_values_map();

    let abilities = ability::build_abilities(abilities_csv).into_id_map();
    let colors = pokemon_color::build_pokemon_colors(pokemon_colors_csv, color_names_map).into_id_map();
    let generations = generation::build_generations(generations_csv, generation_names_map).into_id_map();
    let growth_rates = growth_rate::build_growth_rates(growth_rates_csv, growth_rate_names_map).into_id_map();
    let habitats = pokemon_habitat::build_pokemon_habitats(pokemon_habitats_csv, habitat_names_map).into_id_map();
    let languages = language::build_languages(languages_csv, language_names_map).into_id_map();
    let pokemon = build_pokemon(pokemon_csv).into_id_map();
    let shapes = pokemon_shape::build_pokemon_shapes(pokemon_shapes_csv, data_path).into_id_map();

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
    }
}