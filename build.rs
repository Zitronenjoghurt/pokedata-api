use pokedata_api_build::initialize::build_app_state;
use pokedata_api_utils::directories::data_path;
use pokedata_api_utils::filesystem::create_directory;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    create_data_path();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("data.bin");

    let app_state = build_app_state(&data_path());
    let encoded_state: Vec<u8> = bincode::serialize(&app_state).unwrap();

    let mut file = File::create(dest_path).unwrap();
    file.write_all(&encoded_state).unwrap();

    build_cli();

    println!("cargo:rerun-if-changed=commands/src");
    println!("cargo:rerun-if-changed=data");
}

fn create_data_path() {
    create_directory(&data_path());
}

fn build_cli() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let commands_dir = PathBuf::from(&manifest_dir).join("commands");
    let status = Command::new("cargo")
        .current_dir(&commands_dir)
        .args(&["build", "--release"])
        .status()
        .expect("Failed to build commands");

    if !status.success() {
        panic!("Failed to build commands");
    }

    let src_exe = commands_dir.join("target/release/commands");
    let dest_exe = PathBuf::from(&manifest_dir).join("cli");

    std::fs::copy(src_exe, dest_exe).expect("Failed to copy executable to root directory");
}