use std::path::PathBuf;

use clap::Parser;
use kbcompose::extract_markdown_files_recursive;

#[derive(Parser)]
#[command(version = "0.1.0", author = "Vittorio Distefano", about = "")]
struct Opts {
    #[clap(help = "insert target path")]
    path: PathBuf,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path_buf = PathBuf::from(args.last().unwrap());
    let output = extract_markdown_files_recursive(&path_buf);
    println!("{:#?}", output);
}
