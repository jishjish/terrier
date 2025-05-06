/* Func for CLI:
   - Functions supporting codebase summarizations
   - Match clause should correspond to SUPPORTED_TYPES in utils.rs
        - Rust (rs), Python (py), etc. */

use std::fs;
use std::path::PathBuf;
use owo_colors::OwoColorize;
use tabled::{
    builder::Builder, 
    settings::{
        Style, 
        Padding, 
        Panel,
        Color, 
        object::Rows
    }
};
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
        "js" => {
            println!("{}", "Parsing Javascript file".green());
            Some("function")
        }
        unsupported => {
            // println!("Unsupported file type.");
            assert!(false, "Unsupported file type: {}", unsupported);
            None
        }
    };

    let mut count: i32 = 0;
    for (_i, line) in contents.lines().enumerate() {
        if line.contains(&func_name.unwrap().to_string()) {
            count += 1;
            // println!("Line {}:      {}", i+1, line.blue());
        }
    }

    let file_name = filename.file_name().unwrap().to_string_lossy().to_string();
    let tot_lines = contents.lines().count().to_string();
    let function_count = count.to_string();
    let mut builder = Builder::new();
    builder.push_record(["Total Lines", &tot_lines]);
    builder.push_record(["Functions", &function_count]);

    // Build the table
    let mut table = builder.build();

    // Apply styling
    table
        .with(Style::modern())
        // Add extra padding at the top where our title will go
        .with(Padding::new(1, 0, 0, 0))
        // Create a "title" by adding a panel above the table
        .with(Panel::header(file_name))
        // Make the header row (first row) have a different color
        .modify(Rows::first(), Color::FG_BLUE);

    // Print the table
    println!("{}", table);

    None
}









