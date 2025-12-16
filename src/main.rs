mod args;
mod validator;
mod scan_dir;

use clap::Parser;
use std::path::PathBuf;

use args::Args;
use validator::validate_path;
use scan_dir::print_tree;

fn main() {
    let args = Args::parse();

    let path = args
        .path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    if let Err(err) = validate_path(&path) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    println!("{}", path.display());
    print_tree(&path, "", args.only_file, 0, args.depth);
}