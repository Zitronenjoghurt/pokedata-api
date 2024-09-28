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