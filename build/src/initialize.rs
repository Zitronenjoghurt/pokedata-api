use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::ability::build_abilities;
use pokedata_api_types::entities::api::species::build_species;
use pokedata_api_types::entities::csv::abilities::AbilitiesCSV;
use pokedata_api_types::entities::csv::pokemon_species::PokemonSpeciesCSV;
use pokedata_api_types::entities::csv_entity::{get_download_map, CSVEntity};
use pokedata_api_types::entities::traits::into_id_map::IntoIdMap;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

pub fn initialize_data(data_path: &PathBuf) -> AppState {
    create_data_directory(data_path);
    download_csv_files(data_path);

    let abilities_csv = AbilitiesCSV::load(data_path).unwrap();
    let pokemon_species_csv = PokemonSpeciesCSV::load(data_path).unwrap();

    let abilities = build_abilities(abilities_csv).into_id_map();
    let species = build_species(pokemon_species_csv).into_id_map();

    AppState {
        abilities,
        species,
    }
}

fn create_data_directory(data_path: &PathBuf) {
    if !data_path.exists() {
        std::fs::create_dir_all(data_path).expect("Unable to create data directory");
        println!("cargo:info=Created data directory: {}", data_path.display());
    } else {
        println!("cargo:info=Found data directory: {}", data_path.display());
    }
}

fn download_csv_files(base_path: &PathBuf) {
    let download_map = get_download_map();

    for (file_name, download_url) in download_map {
        let mut file_path = base_path.join(&file_name);
        file_path.set_extension("csv");
        handle_csv_file(file_path, download_url);
    }
}

fn handle_csv_file(file_path: PathBuf, download_url: String) {
    if !file_path.exists() {
        download_csv_file(file_path, download_url);
    } else {
        println!("cargo:info=Found existing csv file: {}", file_path.display());
    }
}

fn download_csv_file(file_path: PathBuf, download_url: String) {
    println!("cargo:info=Downloading csv file: {}", file_path.display());
    println!("cargo:info=Requesting... {}", download_url);

    let client = Client::new();
    let mut response = client.get(&download_url)
        .send()
        .expect("Failed to send request");

    let mut file = File::create(&file_path)
        .expect("Failed to create file");

    copy(&mut response, &mut file)
        .expect("Failed to copy content");

    println!("cargo:info=Finished downloading csv file: {}", file_path.display());
}