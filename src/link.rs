use std::fs;
use regex::Regex;
use walkdir::WalkDir;
use std::path::PathBuf;
// use owo_colors::OwoColorize;
use std::collections::HashMap;
// Internal imports
use crate::utils:: { EXCLUDED_DIRECTORIES, SUPPORTED_TYPES};


pub struct CodeLinkAnalyzer {
    pub file_contents: HashMap<String, (String, String)>,
    pub extracted_functions: HashMap<String, Vec<String>>
}

impl CodeLinkAnalyzer {

    pub fn new() -> Self {
        Self {
            file_contents: HashMap::new(),
            extracted_functions: HashMap::new(),
        }
    }

    /// Iterate through directory, extract supported file types and their contents, store in hashmap
    pub fn file_content_extractor(&mut self, filename: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let walker = WalkDir::new(filename).into_iter();

        let filtered = walker.filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !(EXCLUDED_DIRECTORIES.contains(&name.as_ref()) || name.starts_with("."))
        });

        for entry in filtered {
            let entry = entry?;
            // Only process files 
            if entry.file_type().is_file() {
                // println!("entry is: {:?}", entry);
                if entry.file_type().is_file() {
                    let file_path = entry.path();
                    let ext_type = file_path.extension()
                        .map_or(String::from(""), |ext| ext.to_string_lossy().to_string());
                    
                    if SUPPORTED_TYPES.contains(&&ext_type.as_str()) {
                        let contents: String = fs::read_to_string(file_path)
                            .expect("Unable to read file");
                        self.file_contents.insert(entry.file_name().to_string_lossy().to_string(), (ext_type, contents));
                    }
                }

            }
        }
        Ok(())
    }

    /// Search through file contents and extract functions. Store in extracted_functions
    pub fn function_extractor(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Set regex patterns for different file types
        let python_re = Regex::new(r"def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\((.*?)\)").unwrap();
        let rust_re = Regex::new(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\((.*?)\)").unwrap();

        for (key, value) in &self.file_contents {
            let re = match value.0.as_str() {
                "py" => Some(&python_re),
                "rs" => Some(&rust_re),
                _ => None
            };

            for (_i, matched) in re.unwrap().find_iter(&value.1).enumerate() {
                let matched_function = matched.as_str();
                // Check if key exists, otherwise push to new key in extracted_functions
                self.extracted_functions.entry(key.to_string())
                    .or_insert_with(Vec::new)
                    .push(matched_function.to_string());
            }
        }
        println!("res: {:#?}", self.extracted_functions);
        Ok(())
    }

}


