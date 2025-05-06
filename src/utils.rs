use std::path::PathBuf;
use owo_colors::OwoColorize;

// Files to ignore
pub const EXCLUDED_DIRECTORIES: &[&str] = &[
    "venv",
    "env", 
    "uv.lock",
    "node_modules", 
    "__pycache__",
    "site-packages",
    "dist",
    "build",
    "target",
    ".git"
];

// Supported file types to search in path
pub const SUPPORTED_TYPES: &[&str] = &["py", "rs", "js"];

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
            panic!("File type not supported. Must be one of {:?}", SUPPORTED_TYPES);
            // None
        }
    } else {
        panic!("File not found at: {:?}", filename.red());
    }
}

