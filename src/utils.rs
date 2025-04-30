// use std::path::Path;
// use std::ffi::OsStr;
use std::path::PathBuf;
use owo_colors::OwoColorize;

// Supported file types to search in path
const SUPPORTED_TYPES: &[&str] = &["py", "rs"];

/// Takes arg of filename, extracts extension and checks if file type is supported.
pub fn get_extension_from_filename(filename: PathBuf) -> Option<String> {

    if filename.exists() {
        let extension = filename.extension()?;
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


