use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {

    // /// The file path you want to grep search.
    // #[arg(short, long)]
    // pub path: String,

    /// Files or directories to search
    #[arg(short = 'p', long, value_name = "PATH")]
    pub paths: PathBuf,

    /// The keyword to search for.
    #[arg(short = 'k', long)]
    pub keyword: String,

    // /// Optional flag to print file tree 
    // #[arg(short, long, default_value_t = false)]
    // tree: bool,
}
