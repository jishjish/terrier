// external imports
use clap::Parser;
use owo_colors::OwoColorize;

// import Cli args from args.rs
use terrier::args::Cli;
use terrier::args::Commands;
// Utils (utils.rs)
use terrier::utils::get_extension_from_filename;
// Grep (grep.rs)
use terrier::grep::search_file_for_keyword;
// Func (func.rs)
use terrier::func::func_identification;
// Link (link.rs)
use terrier::link::CodeLinkAnalyzer;


/// Main function that orchestrates cli args
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli: Cli = Cli::parse();
    
    match cli.command {
        Commands::Grep(args) => {
            println!("{}: {:?}", "Searching file".green(), &args.paths);
            println!("{}: {:?}", "Searching for".green(), &args.keyword);
            // Extract the file extension
            get_extension_from_filename(&args.paths);                            
            // Search file for keyword
            search_file_for_keyword(args.keyword, &args.paths);        
        },
        Commands::Func(args) => {
            println!("{}: {:?}", "Finding functions in".green(), &args.paths);
            // Get extension is called inside of build function func 
            // Call func identifier
            func_identification(&args.paths);                         
        },
        Commands::Link(args) => {
            // Instantiate new analyzer
            let mut analyzer = CodeLinkAnalyzer::new();
            // Set args as path
            analyzer.file_content_extractor(&args.paths)?;
            analyzer.function_extractor()?;
            analyzer.overlaps()?;
            analyzer.link_builder()?;
        }
        // Commands::Tree(args) => {
        //     pritnln!("Examining structure of codebase: {}", &args.paths)
        // }
    }
    Ok(())
}
