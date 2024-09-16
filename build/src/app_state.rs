use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::{ability, language, species};
use pokedata_api_types::entities::csv::{abilities, language_names, languages, pokemon_species};
use pokedata_api_types::entities::csv_entity::CSVEntity;
use pokedata_api_types::entities::traits::into_id_map::IntoIdMap;
use pokedata_api_types::entities::traits::into_localized_names_map::IntoLocalizedNames;
use std::path::PathBuf;

pub fn create_app_state(data_path: &PathBuf) -> AppState {
    let abilities_csv = abilities::AbilitiesCSV::load(data_path).unwrap();
    let languages_csv = languages::LanguagesCSV::load(data_path).unwrap();
    let pokemon_species_csv = pokemon_species::PokemonSpeciesCSV::load(data_path).unwrap();

    let language_names_map = language_names::LanguageNamesCSV::load(data_path).unwrap().into_localized_names_map();

    let abilities = ability::build_abilities(abilities_csv).into_id_map();
    let languages = language::build_languages(languages_csv, language_names_map).into_id_map();
    let species = species::build_species(pokemon_species_csv).into_id_map();

    AppState {
        abilities,
        languages,
        species,
    }
}