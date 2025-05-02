use std::fs;
use std::collections::HashMap;
// use std::fs::read_dir;
use std::path::PathBuf;
use owo_colors::OwoColorize;

use walkdir::WalkDir;
use crate::utils::SUPPORTED_TYPES;

/// Iteration over folder to identify function interconnectedness.
/// Pass in folder / structure to iterate through.
pub fn link_func_search(filename: &PathBuf) -> HashMap<std::string::String, std::string::String> {

    // let mut supported_files: Vec<PathBuf> = Vec::new();

    // let mut supported_files = HashMap::new();
    let mut map = HashMap::new();


    for entry in WalkDir::new(filename) {
        let entry = entry.unwrap();
        // println!("{}", entry.path().display());
        // println!("entry: {:?}", entry.path());

        if entry.file_type().is_file() {
            println!("file: {:?}", entry);

            let file_path = entry.path();

            let extension = file_path.extension().unwrap();
            let ext_type = extension.to_string_lossy().to_string();
            // println!("extension is {:?}", extension);

            if SUPPORTED_TYPES.contains(&ext_type.as_str()) {
                let contents: String = fs::read_to_string(file_path)
                    .expect("Unable to read file.");

                // println!("{}", "---------------------------------".red());
                // println!("{}", contents);
                // println!("{}", "---------------------------------".red());

                map.insert(entry.file_name().to_string_lossy().to_string(), contents);

            }
        }

    }

    println!("map is {:?}", map);
    map 

}