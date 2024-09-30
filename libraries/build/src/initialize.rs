use crate::app_state::create_app_state;
use git2::Repository;
use pokedata_api_entities::app_state::AppState;
use pokedata_api_parsing::csv_entity::get_download_map;
use pokedata_api_utils::constants::POKEMON_TCG_REPOSITORY_GIT_URL;
use pokedata_api_utils::directories::tcg_repository_path;
use pokedata_api_utils::filesystem::create_directory;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

pub fn build_app_state(data_path: &PathBuf) -> AppState {
    let csv_path = data_path.join("csv");
    create_directory(&csv_path);
    download_csv_files(&csv_path);
    prepare_pokemon_tcg_repository();
    create_app_state(&csv_path)
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
        println!("Found existing csv file: {}", file_path.display());
    }
}

fn download_csv_file(file_path: PathBuf, download_url: String) {
    println!("Downloading csv file: {}", file_path.display());
    println!("Requesting... {}", download_url);

    let client = Client::new();
    let mut response = client.get(&download_url)
        .send()
        .expect("Failed to send request");

    let mut file = File::create(&file_path)
        .expect("Failed to create file");

    copy(&mut response, &mut file)
        .expect("Failed to copy content");

    println!("Finished downloading csv file: {}", file_path.display());
}

fn prepare_pokemon_tcg_repository() {
    let repo_path = tcg_repository_path();

    if !repo_path.exists() {
        clone_pokemon_tcg_repository(&repo_path);
    } else {
        update_pokemon_tcg_repository(&repo_path);
    }
}

fn clone_pokemon_tcg_repository(repo_path: &PathBuf) {
    create_directory(&repo_path);
    println!("Cloning tcg repository... (this might take a bit)");
    match Repository::clone(POKEMON_TCG_REPOSITORY_GIT_URL, &repo_path) {
        Ok(_) => println!("Successfully cloned tcg repository"),
        Err(e) => panic!("Failed to clone tcg repository: {}", e)
    }
}

fn update_pokemon_tcg_repository(repo_path: &PathBuf) {
    let repo = Repository::open(repo_path).unwrap();
    let mut remote = repo.find_remote("origin").unwrap();
    remote.fetch(&["main"], None, None).unwrap();
    println!("Successfully updated tcg repository")
}