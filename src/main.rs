#![feature(rustc_private)]

use std::{env, fs, path::PathBuf};
mod thir_analysis;
use walkdir::WalkDir;

fn get_rust_files(folder_path: &PathBuf) -> Vec<PathBuf> {
    let mut rust_files = Vec::new();

    for entry in WalkDir::new(folder_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "rs") {
            rust_files.push(path.to_path_buf());
        }
    }

    rust_files
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &String::from(&args[1]);
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");

    // TODO: think about ideal distribution of file_name and contents
    thir_analysis::thir_analysis(file_name, contents);
}
