use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    Grep(GrepArgs),
    Tree(TreeArgs),
}

#[derive(Debug, Parser)]
pub struct GrepArgs {
    #[arg(short = 'p', long)]
    pub paths: PathBuf,
    
    #[arg(short = 'k', long)]
    pub keyword: String,
}

#[derive(Debug, Parser)]
pub struct TreeArgs {
    #[arg(short = 'p', long)]
    pub paths: PathBuf,
}