use std::fs;
use std::fs::read_dir;
use std::path::PathBuf;

use walkdir::WalkDir;
use crate::utils::SUPPORTED_TYPES;

/// Iteration over folder to identify function interconnectedness.
/// Pass in folder / structure to iterate through.
pub fn link_func_search(filename: &PathBuf) {

    // println!("supported types are : {:?}", SUPPORTED_TYPES);

    let mut supported_files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(filename) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());

        // println!("dir entry is {:?}", entry);

        
        if entry.file_type().is_dir() {
            ;
        } else {
            let contents: String = fs::read_to_string(filename)
                .expect("Unable to read file.");
            println!("contents are: {}", contents);
        }
    }

}