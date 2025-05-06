use std::fs;
use std::path::PathBuf;
// use owo_colors::OwoColorize;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
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

pub fn search_file_for_keyword(keyword: String, filename: &PathBuf) -> Option<String> {
    let contents: String = fs::read_to_string(filename)
        .expect("Unable to read file.");

    let mut count: i32 = 0;
    let matcher: SkimMatcherV2 = SkimMatcherV2::default();

    for (_i, line) in contents.lines().enumerate() {
        // Normalize line and keyword to lowercase for case 
        // insensitive reference
        if matcher.fuzzy_match(line, &keyword).is_some() {
            count += 1;
            // println!("Line {} in {:?}: {}", i+1, filename, line.blue());
        }
    }

    let file_name = filename.file_name().unwrap().to_string_lossy().to_string();
    let query = keyword + " in " + &file_name;
    let tot_lines = contents.lines().count().to_string();
    let matches = count.to_string();
    let match_density = format!("{:.2}%", (count as f64 / contents.lines().count() as f64) * 100.0);

    // Set builder for output 
    let mut builder = Builder::new();
    builder.push_record(["Total Lines", &tot_lines]);
    builder.push_record(["Matches", &matches]);
    builder.push_record(["Match Density", &match_density]);

    // Build the table
    let mut table = builder.build();

    // Apply styling
    table
        .with(Style::modern())
        // Add extra padding at the top where our title will go
        .with(Padding::new(1, 0, 0, 0))
        // Create a "title" by adding a panel above the table
        .with(Panel::header(query))
        // Make the header row (first row) have a different color
        .modify(Rows::first(), Color::FG_BLUE);

    // Print the table
    println!("{}", table);

    None
}
