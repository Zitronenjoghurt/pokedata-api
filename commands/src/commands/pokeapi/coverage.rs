use crate::commands::pokeapi::{CSV_DATA_PATH, IGNORE_FILE_PREFIXES, POKEAPI_OWNER, POKEAPI_REPO};
use pokedata_api_parsing::csv_entity::get_all_metadata;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct GitHubFile {
    name: String,
}

pub fn coverage() -> Result<(), String> {
    let url = format!("https://api.github.com/repos/{}/{}/contents/{}", POKEAPI_OWNER, POKEAPI_REPO, CSV_DATA_PATH);
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str("Rust reqwest").unwrap(),
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .unwrap();

    let all_files: Vec<GitHubFile> = if response.status().is_success() {
        response.json().unwrap()
    } else {
        return Err(format!("An error occurred while requesting the GitHub API: {}", response.status()));
    };

    let csv_files: Vec<String> = all_files
        .into_iter()
        .map(|file| file.name)
        .filter(|name| {
            name.ends_with(".csv") &&
                !IGNORE_FILE_PREFIXES.iter().any(|&prefix| name.starts_with(prefix))
        })
        .collect();

    let covered_files: Vec<String> = get_all_metadata().iter().map(|data| format!("{}.csv", data.file_name)).collect();

    let total_files = csv_files.len();
    let covered_count = covered_files.len();
    let uncovered_files: Vec<String> = csv_files
        .into_iter()
        .filter(|name| !covered_files.contains(name))
        .collect();

    println!("Coverage: {}/{} files", covered_count, total_files);
    println!("Uncovered files:");
    for file in &uncovered_files {
        println!("{}", file);
    }

    Ok(())
}