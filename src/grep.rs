use std::fs;
use std::path::PathBuf;
use owo_colors::OwoColorize;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub fn search_file_for_keyword(keyword: String, filename: &PathBuf) -> Option<String> {
    let contents: String = fs::read_to_string(filename)
        .expect("Unable to read file.");

    let mut count: i32 = 0;
    let matcher: SkimMatcherV2 = SkimMatcherV2::default();

    for (i, line) in contents.lines().enumerate() {
        // Normalize line and keyword to lowercase for case 
        // insensitive reference
        if matcher.fuzzy_match(line, &keyword).is_some() {
            count += 1;
            println!("Line {} in {:?}: {}", i+1, filename, line.blue());
        }
    }
    println!("┌──────── Summary: {:?} ────────────┐", filename.file_name().unwrap().red());
    println!("│  - Total lines: {:<20}   │", contents.lines().count().green());
    println!("│  - Matches: {:<24}   │", count.to_string().green());
    println!("│  - Match density(%): {:<18.2}│", ((count as f64 / contents.lines().count() as f64) * 100.0).green());
    println!("└────────────────────────────────────────┘");

    None
}
