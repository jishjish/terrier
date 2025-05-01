// external imports
use clap::Parser;
use owo_colors::OwoColorize;

// mods
mod args;
mod utils;
mod grep;
mod func;

// import Cli args from args.rs
use args::Cli;
use crate::args::Commands;
use utils::get_extension_from_filename;

// Grep (grep.rs)
use grep::search_file_for_keyword;

// Func (func.rs)
use func::func_identification;


fn main() {
    let cli: Cli = Cli::parse();
    
    match cli.command {
        Commands::Grep(args) => {
            println!("************************************************");
            println!("{}: {:?}", "Searching file".green(), &args.paths);
            println!("{}: {:?}", "Searching for".green(), &args.keyword);
            println!("************************************************");
            // Extract the extension
            get_extension_from_filename(&args.paths);
            // Search the file for keyword
            search_file_for_keyword(args.keyword, &args.paths);
        },
        Commands::Func(args) => {
            // println!("************************************************");
            println!("{}: {:?}", "Finding functions in".green(), &args.paths);
            // println!("************************************************");
            // Extension is called inside of build function func 
            // Call func identifier
            func_identification(&args.paths);
        }
    }
}


