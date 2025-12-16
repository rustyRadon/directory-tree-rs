use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Path to directory (current folder = default)
    #[arg(value_name = "PATH")]
    pub path: Option<String>,

    /// Max depth to be displayed 
    #[arg(short = 'd', long = "depth", default_value_t = usize::MAX)]
    pub depth: usize,

    /// Show only files, no directories
    #[arg(short, long)]
    pub only_file: bool,
}