use std::path::PathBuf;

pub fn create_directory(path: &PathBuf) {
    if !path.exists() {
        std::fs::create_dir_all(path).expect(&format!("Unable to create directory: {}", path.display()));
        println!("Created directory: {}", path.display());
    } else {
        println!("Found directory: {}", path.display());
    }
}

pub fn get_data_path() -> Option<PathBuf> {
    match dirs::data_dir() {
        Some(dir) => Some(dir.join("pokedata-api")),
        None => None
    }
}