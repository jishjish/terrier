use std::fs;
use regex::Regex;
use walkdir::WalkDir;
use std::path::PathBuf;
use std::collections::HashMap;
// Internal imports
use crate::utils::SUPPORTED_TYPES;


// /// Iteration over directory, extract file and code.
// fn file_content_extractor(filename: &PathBuf) -> HashMap<std::string::String, std::string::String> {

//     // Set hashmap for file name and contents
//     let mut map: HashMap<String, String> = HashMap::new();

//     for entry in WalkDir::new(filename) {
//         let entry = entry.unwrap();

//         if entry.file_type().is_file() {
//             println!("file: {:?}", entry);

//             let file_path = entry.path();

//             let extension = file_path.extension().unwrap();
//             let ext_type = extension.to_string_lossy().to_string();

//             if SUPPORTED_TYPES.contains(&ext_type.as_str()) {
//                 let contents: String = fs::read_to_string(file_path)
//                     .expect("Unable to read file.");
//                 map.insert(entry.file_name().to_string_lossy().to_string(), contents);
//             }
//         }
//     }
//     // Return map
//     map 
// }



// fn function_extractor(file_contents: HashMap<String, String>) {

// } 



// pub fn link_func_search(filename: &PathBuf) {
//     let contents: HashMap<String, String> = file_content_extractor(filename);

//     // function_extractor(contents);

// }



// -------

pub struct CodeLinkAnalyzer {
    pub file_contents: HashMap<String, String>,
    pub extracted_functions: HashMap<String, String>
}

impl CodeLinkAnalyzer {

    pub fn new() -> Self {
        Self {
            file_contents: HashMap::new(),
            extracted_functions: HashMap::new(),
        }
    }

    // Iterate through directory, extract supported file types and their contents, store in hashmap
    pub fn file_content_extractor(&mut self, filename: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        for entry in WalkDir::new(filename) {
            let entry = entry.unwrap();
    
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let extension = file_path.extension().unwrap();
                let ext_type = extension.to_string_lossy().to_string();
    
                if SUPPORTED_TYPES.contains(&ext_type.as_str()) {
                    let contents: String = fs::read_to_string(file_path)
                        .expect("Unable to read file.");
                    // Insert into file_contents struct
                    self.file_contents.insert(entry.file_name().to_string_lossy().to_string(), contents);
                }
            }
        }
        Ok(())
    }

    pub fn function_extractor(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        
        let python_re = Regex::new(r"def\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\((.*?)\)").unwrap();
        let rust_re = Regex::new(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\((.*?)\)").unwrap();

        for (key, value) in &self.file_contents {
            println!("key is: {}", key);
            println!("value is: {}", value);
        }
        Ok(())
    }

    pub fn test(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut path = PathBuf::new();
        // path.push(r"/Users/me/Desktop/rust/aaa/src");
        path.push("/Users/joshphillips/Desktop/rust/terrier/src");
        self.file_content_extractor(&path)
    }
}


