use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;
// Internal imports
use crate::utils::SUPPORTED_TYPES;


/// Iteration over directory, extract file and code.
fn file_content_extractor(filename: &PathBuf) -> HashMap<std::string::String, std::string::String> {

    // Set hashmap for file name and contents
    let mut map: HashMap<String, String> = HashMap::new();

    for entry in WalkDir::new(filename) {
        let entry = entry.unwrap();

        if entry.file_type().is_file() {
            println!("file: {:?}", entry);

            let file_path = entry.path();

            let extension = file_path.extension().unwrap();
            let ext_type = extension.to_string_lossy().to_string();

            if SUPPORTED_TYPES.contains(&ext_type.as_str()) {
                let contents: String = fs::read_to_string(file_path)
                    .expect("Unable to read file.");
                map.insert(entry.file_name().to_string_lossy().to_string(), contents);
            }
        }
    }
    // Return map
    map 
}



fn function_extractor(file_contents: HashMap<String, String>) {

} 



pub fn link_func_search(filename: &PathBuf) {
    let contents: HashMap<String, String> = file_content_extractor(filename);

    // function_extractor(contents);

}