use std::fs;
use std::path::PathBuf;
use owo_colors::OwoColorize;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;


/* Grep Based - functions
   supporting grep commands */

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

    let mut count = 0;
    let matcher = SkimMatcherV2::default();

    for (i, line) in contents.lines().enumerate() {
        // Normalize line and keyword to lowercase for case 
        // insensitive reference
        if matcher.fuzzy_match(line, &keyword).is_some() {
            count += 1;
            println!("Line {} in {:?}: {}", i+1, filename, line.blue());
        }
    }
    println!("┌──────── Summary: {:?} ────────────┐", filename.file_name().unwrap().red());
    println!("│  - Total lines: {:<20}   │", contents.lines().count());
    println!("│  - Matches: {:<24}   │", count.to_string().green());
    println!("│  - Match density(%): {:<18.2}│", (count as f64 / contents.lines().count() as f64) * 100.0);
    println!("└────────────────────────────────────────┘");

    None
}




/* Tree Based - functions
   supporting grep commands */

pub fn build_function_tree(filename: &PathBuf) -> Option<String> {
    let ext = get_extension_from_filename(filename)?;
    let contents = fs::read_to_string(filename)
        .expect("Unable to read file.");

    // match function based on file type
    match ext.as_str() {
        "rs" => {
            println!("{}", "Parsing Rust file...".green());
            let mut count = 0;
            for (i, line) in contents.lines().enumerate() {
                if line.contains("fn") {
                    count += 1;
                    println!("Line {} in {:?}: {}", i+1, filename, line.blue());
                    println!("{}", count);
                }
            }
        },
        "py" => {
            println!("{}", "Parsing Python file...".green());
        },
        _ => {
            println!("unknown file type")
        }
    }
    None
}