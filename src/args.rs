use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {

    /// The file path you want to grep search.
    #[arg(short, long)]
    pub path: String,

    /// The keyword to search for.
    #[arg(short, long)]
    pub keyword: String,
}
