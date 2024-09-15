use pokedata_api_build::initialize::initialize_data;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let data_path = PathBuf::from(out_dir).join("data");

    let app_state = initialize_data(&data_path);
    let encoded_data: Vec<u8> = bincode::serialize(&app_state).unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("data.bin");

    let mut file = File::create(dest_path).unwrap();
    file.write_all(&encoded_data).unwrap();
}