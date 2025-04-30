use std::fs;
use std::path::PathBuf;
use owo_colors::OwoColorize;

// Supported file types to search in path
const SUPPORTED_TYPES: &[&str] = &["py", "rs"];

/// Takes arg of filename, extracts extension and checks if file type is supported.
pub fn get_extension_from_filename(filename: &PathBuf) -> Option<String> {
    if filename.exists() {
        // Extract the extension from filename
        let extension = filename.extension()?;

        // Convert extension to string 
        let ext_str = extension.to_string_lossy().to_string();

        if SUPPORTED_TYPES.contains(&ext_str.as_str()) {
            Some(ext_str)
        } else {
            println!("File type not supported. Must be one of {:?}", SUPPORTED_TYPES);
            None
        }
    } else {
        panic!("File not found at: {:?}", filename.red());
    }
}



pub fn search_file_for_keyword(keyword: String, filename: &PathBuf) -> Option<String> {
    let contents = fs::read_to_string(filename)
        .expect("Unable to read file.");

    for (i, line) in contents.lines().enumerate() {
        // Normalize line and keyword to lowercase for case 
        // insensitive reference
        if line.to_lowercase().contains(&keyword.to_lowercase()) {
            println!("Line {} in {:?}: {}", i+1, filename, line.blue());
        }
    }
    None
}