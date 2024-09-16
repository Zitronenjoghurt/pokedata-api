use pokedata_api_build::initialize::build_app_state;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let data_path = PathBuf::from(out_dir).join("data");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("data.bin");

    let app_state = build_app_state(&data_path);
    let encoded_state: Vec<u8> = bincode::serialize(&app_state).unwrap();

    let mut file = File::create(dest_path).unwrap();
    file.write_all(&encoded_state).unwrap();
}