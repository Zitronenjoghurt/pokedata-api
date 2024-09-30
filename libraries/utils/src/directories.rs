use std::path::PathBuf;

pub fn data_path() -> PathBuf {
    PathBuf::from("./data")
}

pub fn sprite_indices_path() -> PathBuf {
    PathBuf::from("./sprite_indices")
}

pub fn configuration_path() -> PathBuf {
    PathBuf::from("./configuration")
}

pub fn tcg_repository_path() -> PathBuf {
    data_path().join("pokemon-tcg")
}

pub fn tcg_repository_cards_path() -> PathBuf {
    tcg_repository_path().join("cards").join("en")
}

pub fn tcg_repository_decks_path() -> PathBuf {
    tcg_repository_path().join("decks").join("en")
}

pub fn tcg_repository_sets_path() -> PathBuf {
    tcg_repository_path().join("sets")
}