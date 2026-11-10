mod args;
mod markdown;

use args::Args;
use clap::Parser;
use markdown::MarkdownFile;
use std::{fs, io};
use walkdir::WalkDir;

fn is_text(text: &str) -> bool {
    !text
        .chars()
        .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
}

fn main() -> io::Result<()> {
    let mut args = Args::parse();

    if args.output.extension().is_none() {
        args.output.set_extension("md");
    }

    let mut treepack_file = MarkdownFile::new(&args.output)?;

    for entry in WalkDir::new(&args.directory).sort_by_file_name() {
        let Ok(entry) = entry else {
            continue;
        };

        if !entry.file_type().is_file() || treepack_file.path() == entry.path().canonicalize()? {
            continue;
        }

        let Ok(content) = fs::read_to_string(entry.path()) else {
            continue;
        };

        if is_text(&content) {
            treepack_file.append(
                entry
                    .path()
                    .strip_prefix(&args.directory)
                    .unwrap_or(entry.path()),
                &content,
            )?;
        }
    }

    Ok(())
}
