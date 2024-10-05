use crate::commands::pokeapi::{CSV_DATA_PATH, IGNORE_FILE_PREFIXES, POKEAPI_OWNER, POKEAPI_REPO};
use pokedata_api_parsing::csv_entity::get_all_metadata;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use std::collections::HashSet;

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

    let mut skipped_files_count = 0;
    let csv_files: Vec<String> = all_files
        .into_iter()
        .map(|file| file.name)
        .filter(|name| {
            let is_csv = name.ends_with(".csv");
            if is_csv && IGNORE_FILE_PREFIXES.iter().any(|&prefix| name.starts_with(prefix)) {
                skipped_files_count += 1;
                false
            } else {
                is_csv
            }
        })
        .collect();

    let covered_files: HashSet<String> = get_all_metadata().iter().map(|data| format!("{}.csv", data.file_name)).collect();

    let csv_files_count = csv_files.len();
    let covered_files_count = covered_files.len();
    let uncovered_files: Vec<String> = csv_files
        .into_iter()
        .filter(|name| !covered_files.contains(name))
        .collect();

    let percentage_covered = (covered_files_count as f32 / csv_files_count as f32) * 100.0;

    println!("Coverage: {}/{} files ({:.2}%)", covered_files_count, csv_files_count, percentage_covered);
    println!("Skipped files: {}", skipped_files_count);
    println!("Uncovered files:");
    for file in &uncovered_files {
        println!("{}", file);
    }

    Ok(())
}