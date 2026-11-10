use clap::Parser;
use std::path::PathBuf;

/// Recursively collects the contents of UTF-8 text files into a Markdown file
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Directory to scan. Defaults to the current working directory
    #[arg(short, long, default_value = ".")]
    directory: PathBuf,

    /// Path to the output Markdown file
    #[arg(short, long, default_value = "treepack.md")]
    output: String,
}

fn main() {
    let args = Args::parse();

    println!("Directory: {:#?}\nOutput: {}", args.directory, args.output)
}
