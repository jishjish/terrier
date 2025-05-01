/* Func for CLI:
   - Functions supporting codebase summarizations
   - Match clause should correspond to SUPPORTED_TYPES in utils.rs
        - Rust (rs), Python (py), etc. */

use std::fs;
use std::collections;
use std::path::PathBuf;
use owo_colors::OwoColorize;
use crate::utils::get_extension_from_filename;  // internal fn from utils


// from given filename, extract all functions and
pub fn func_identification(filename: &PathBuf) -> Option<String> {
    let ext = get_extension_from_filename(filename)?;
    let contents = fs::read_to_string(filename)
        .expect("Unable to read file.");

    let func_declarations = collections::HashMap::from([
        ("rs", "fn"),
        ("py", "def")
    ]);


    let func_name: Option<&str> = match ext.as_str() {
        "rs" => {
            println!("{}", "Parsing Rust file...".green());
            Some(func_declarations.get("rs")?)
        },
        "py" => {
            println!("{}", "Parsing Python file...".green());
            Some(func_declarations.get("py")?)
        },
        _ => {
            println!("Unsupported file type.");
            None
        }
    };

    println!("func name is {:?}", func_name.unwrap().to_string());

    let mut count = 0;
    for (i, line) in contents.lines().enumerate() {
        if line.contains("fn") {
            count += 1;
            println!("Line {}:      {}", i+1, line.blue());
        }
    }
    println!("┌──────── Summary: {:?} ────────────┐", filename.file_name().unwrap().red());
    println!("│  - Total lines: {:<20}   │", contents.lines().count().green());
    println!("│  - Functions: {:<24} │", count.to_string().green());
    println!("└────────────────────────────────────────┘");

    // // match function based on file type
    // match ext.as_str() {
    //     "rs" => {
    //         println!("{}", "Parsing Rust file...".green());
    //         let mut count = 0;
    //         for (i, line) in contents.lines().enumerate() {
    //             if line.contains("fn") {
    //                 count += 1;
    //                 println!("Line {}:      {}", i+1, line.blue());
    //             }
    //         }
    //         println!("┌──────── Summary: {:?} ────────────┐", filename.file_name().unwrap().red());
    //         println!("│  - Total lines: {:<20}   │", contents.lines().count().green());
    //         println!("│  - Functions: {:<24} │", count.to_string().green());
    //         println!("└────────────────────────────────────────┘");
        
    //     },
    //     "py" => {
    //         println!("{}", "Parsing Python file...".green());
    //         let mut count = 0;
    //         for (i, line) in contents.lines().enumerate() {
    //             if line.contains("def") {
    //                 count += 1;
    //                 println!("Line {}:      {}", i+1, line.blue());
    //             }
    //         }
    //         println!("┌──────── Summary: {:?} ────────────┐", filename.file_name().unwrap().red());
    //         println!("│  - Total lines: {:<20}   │", contents.lines().count().green());
    //         println!("│  - Functions: {:<24} │", count.to_string().green());
    //         println!("└────────────────────────────────────────┘");
    //     },
    //     _ => {
    //         println!("Unsupported file type")
    //     }
    // }
    None
}