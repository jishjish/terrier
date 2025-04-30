// outside imports
use clap::Parser;
use owo_colors::OwoColorize;

// mods
mod args;
mod utils;

// import Cli args from args.rs
use args::Cli;
use utils::get_extension_from_filename;
use utils::search_file_for_keyword;


fn main() {
    let args: Cli = Cli::parse();
    println!("{}: {:?}", "Searching file".green(), &args.paths);
    println!("{}: {:?}", "Searching for".green(), &args.keyword);

    get_extension_from_filename(&args.paths);
    search_file_for_keyword(args.keyword, &args.paths);
}



