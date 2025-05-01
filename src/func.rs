/* Func for CLI:
   - Functions supporting codebase summarizations
   - Match clause should correspond to SUPPORTED_TYPES in utils.rs
        - Rust (rs), Python (py), etc. */

use std::fs;
use std::path::PathBuf;
use owo_colors::OwoColorize;
use crate::utils::get_extension_from_filename;  // internal fn from utils


// from given filename, extract all functions and return summary
pub fn func_identification(filename: &PathBuf) -> Option<String> {
    let ext = get_extension_from_filename(filename)?;
    let contents = fs::read_to_string(filename)
        .expect("Unable to read file.");


    let func_name: Option<&str> = match ext.as_str() {
        // check for rust
        "rs" => {
            println!("{}", "Parsing Rust file...".green());
            Some("fn")
        },
        // check for python
        "py" => {
            println!("{}", "Parsing Python file...".green());
            Some("def")
        },
        _ => {
            println!("Unsupported file type.");
            None
        }
    };

    let mut count: i32 = 0;
    for (i, line) in contents.lines().enumerate() {
        if line.contains(&func_name.unwrap().to_string()) {
            count += 1;
            println!("Line {}:      {}", i+1, line.blue());
        }
    }

    println!("┌──────── Summary: {:?} ────────────┐", filename.file_name().unwrap().red());
    println!("│  - Total lines: {:<20}   │", contents.lines().count().green());
    println!("│  - Functions: {:<24} │", count.to_string().green());
    println!("└────────────────────────────────────────┘");

    None
}









