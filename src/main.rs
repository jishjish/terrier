// outside imports
use clap::Parser;
use owo_colors::OwoColorize;

// mods
mod args;
mod utils;

// import Cli args from args.rs
use args::Cli;
use crate::args::Commands;
use utils::get_extension_from_filename;

// Grep
use utils::search_file_for_keyword;

// Tree
use utils::build_function_tree;




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
        Commands::Tree(args) => {
            println!("************************************************");
            println!("{}: {:?}", "Building tree for".green(), &args.paths);
            println!("************************************************");
            // Extension is called inside of build function tree
            // Call tree function builder
            build_function_tree(&args.paths);
        }
    }
}


