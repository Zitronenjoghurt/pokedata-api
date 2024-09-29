use crate::app_state::create_app_state;
use pokedata_api_entities::app_state::AppState;
use pokedata_api_parsing::csv_entity::get_download_map;
use pokedata_api_utils::filesystem::create_directory;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

pub fn build_app_state(data_path: &PathBuf) -> AppState {
    let csv_path = data_path.join("csv");
    create_directory(&csv_path);
    download_csv_files(&csv_path);
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