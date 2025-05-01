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
    Func(FuncArgs),
    Link(LinkArgs)
    // Tree(TreeArgs),
}

#[derive(Debug, Parser)]
pub struct GrepArgs {
    #[arg(short = 'p', long)]
    pub paths: PathBuf,
    
    #[arg(short = 'k', long)]
    pub keyword: String,
}

#[derive(Debug, Parser)]
pub struct FuncArgs {
    #[arg(short = 'p', long)]
    pub paths: PathBuf,
}

#[derive(Debug, Parser)]
pub struct LinkArgs {
    #[arg(short = 'p', long)]
    pub paths: PathBuf,
}


// #[derive(Debug, Parser)]
// pub struct TreeArgs {
//     #[arg(short = 'p', long)]
//     pub paths: PathBuf,
// }