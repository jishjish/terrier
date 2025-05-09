use std::fs;
use regex::Regex;
use walkdir::WalkDir;
use std::path::PathBuf;
// use owo_colors::OwoColorize;
use std::collections::{BTreeMap, HashMap};
use tabled::{builder::Builder, settings::Style};
use tabled::settings::{Color, object::Rows};
use owo_colors::OwoColorize;

// Internal imports for dir to exclude / supported file types
use crate::utils:: { EXCLUDED_DIRECTORIES, SUPPORTED_TYPES};

pub struct CodeLinkAnalyzer {
    pub file_contents: HashMap<String, (String, String)>,      // file path, (file type, file contents)
    pub extracted_functions: HashMap<String, Vec<String>>,     // file path, [functions]
    call_graph: BTreeMap<String, Vec<String>>,                 // file::function name [files where referenced]
}

impl CodeLinkAnalyzer {

    pub fn new() -> Self {
        Self {
            file_contents: HashMap::new(),
            extracted_functions: HashMap::new(),
            call_graph: BTreeMap::new(),
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
        let javascript_re = Regex::new(r"function\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\((.*?)\)").unwrap();

        for (key, value) in &self.file_contents {
            let re = match value.0.as_str() {
                "py" => Some(&python_re),
                "rs" => Some(&rust_re),
                "js" => Some(&javascript_re),
                unsupported => {
                    eprintln!("Warning: Unsupported file type '{}', skipping file: {}", unsupported, key);
                    continue;
                }
            };

            // Iterate through, push HashMap<file path, (file type, contents)>
            for (_i, matched) in re.unwrap().find_iter(&value.1).enumerate() {
                let matched_function = matched.as_str();
                // Check if key exists, otherwise push to new key in extracted_functions
                self.extracted_functions.entry(key.to_string())
                    .or_insert_with(Vec::new)
                    .push(matched_function.to_string());
            }
        }
        // println!("res: {:#?}", self.extracted_functions);
        Ok(())
    }


    /// Establish where functions overlap in other files.
    pub fn overlaps(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        // Start for loop on file_contents
        for (file_path, (_file_type, content)) in &self.file_contents {
            // Inner loop over extracted_functions
            for (file, functions) in &self.extracted_functions {
                // Loop over functions in extracted_functions vec
                for func in functions {
                    let pattern = format!(r"{}[\s\(]", regex::escape(func));

                    if let Ok(re) = Regex::new(&pattern) {
                        if re.is_match(content) {
                            let key = format!("{}::{}", file, func);
                            // Push data to call graph struct
                            self.call_graph.entry(key)
                                .or_insert_with(Vec::new)
                                .push(file_path.to_string());
                        }
                    }
                }
            }
        };
        // println!("call graph {:#?}", &self.call_graph);
        Ok(())
    }


    pub fn link_builder(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        let mut builder = Builder::new();
    
        // Add header row 
        builder.push_record(["Source File", "Function Name", "References"]);
        
        // Process each entry in the call graph
        for (key, references) in &self.call_graph {
            // Split the key into source file and function name
            let parts: Vec<&str> = key.split("::").collect();
            if parts.len() >= 2 {
                let source_file = parts[0];
                let function_name = parts[1];
                
                // Join references with commas
                let references_str = references.join(", ");

                // Add a row to the table
                builder.push_record([source_file, function_name, &references_str]);
            }
        }
        
        // Build the table
        let mut table = builder.build();
        
        // Apply styling
        table.with(Style::ascii_rounded());
        table.modify(Rows::first(), Color::FG_BLUE | Color::BOLD);
        
        // Print the table
        println!("{}", table);
        Ok(())
    }

}

