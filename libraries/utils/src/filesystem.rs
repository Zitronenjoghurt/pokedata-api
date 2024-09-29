use std::path::PathBuf;

pub fn create_directory(path: &PathBuf) {
    if !path.exists() {
        std::fs::create_dir_all(path).expect(&format!("Unable to create directory: {}", path.display()));
        println!("Created directory: {}", path.display());
    } else {
        println!("Found directory: {}", path.display());
    }
}

pub fn get_file_name_without_extension(entry: &std::fs::DirEntry) -> Option<String> {
    entry.path()
        .file_stem()
        .and_then(|s| s.to_str())
        .map(String::from)
}

pub fn panic_if_not_exists(path: &PathBuf, context: &str) {
    if !path.exists() {
        panic!("File or directory does not exist: {}\n{}", path.display(), context);
    }
}