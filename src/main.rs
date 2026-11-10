mod args;
mod markdown;

use args::Args;
use clap::Parser;
use std::{fs, io, path::Path};
use walkdir::WalkDir;

fn is_text_utf8(path: &Path) -> io::Result<bool> {
    let bytes = fs::read(path)?;

    let Ok(text) = std::str::from_utf8(&bytes) else {
        return Ok(false);
    };

    let is_binary = text
        .chars()
        .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'));

    Ok(!is_binary)
}

fn main() {
    let args = Args::parse();

    println!(
        "Directory: {}\nOutput: {}",
        args.directory.display(),
        args.output
    );

    for entry in WalkDir::new(args.directory) {
        let Ok(entry) = entry else {
            continue;
        };

        if !entry.file_type().is_file() {
            continue;
        }

        match is_text_utf8(entry.path()) {
            Ok(true) => println!("Text: {}", entry.path().display()),
            Ok(false) => println!("Binary: {}", entry.path().display()),
            Err(err) => eprintln!("Error: {err}"),
        }
    }
}
