use crate::commands::pokeapi::POKEAPI_SPRITES_REPO_HTTPS;
use git2::Repository;
use pokedata_api_utils::filesystem::{create_directory, get_data_path};
use std::path::PathBuf;

pub fn build_sprites() -> Result<(), String> {
    let data_path = get_data_path().unwrap();
    let sprites_path = data_path.join("sprites");
    if !sprites_path.exists() {
        clone_repository(&sprites_path)?;
    } else {
        println!("Found cloned sprites repository");
    }

    // ToDo: analyze files

    Ok(())
}

pub fn clone_repository(sprites_path: &PathBuf) -> Result<(), String> {
    create_directory(&sprites_path);
    println!("Cloning sprites repository... (this might take a bit)");
    match Repository::clone(POKEAPI_SPRITES_REPO_HTTPS, &sprites_path) {
        Ok(_) => println!("Successfully cloned sprites repository"),
        Err(e) => return Err(format!("Failed to clone sprites repository: {}", e))
    };

    Ok(())
}