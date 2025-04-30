// outside imports
use clap::Parser;

// mods
mod args;
mod utils;

// import Cli args from args.rs
use args::Cli;
use utils::get_extension_from_filename;



fn main() {
    let args: Cli = Cli::parse();
    println!("Searching file: {:?}", args.path);
    println!("Keyword searching: {}", args.keyword);

    get_extension_from_filename(&args.path);

}



