mod args;
mod markdown;
mod walker;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("Directory: {:#?}\nOutput: {}", args.directory, args.output)
}
