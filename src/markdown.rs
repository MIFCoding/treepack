use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

pub struct MarkdownFile {
    file: File,
    path: PathBuf,
}

impl MarkdownFile {
    pub fn new(path: &Path) -> io::Result<Self> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        writeln!(file, "# Treepack v{}", env!("CARGO_PKG_VERSION"))?;

        Ok(Self {
            file,
            path: path.canonicalize()?,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn append(&mut self, path: &Path, content: &str) -> io::Result<()> {
        writeln!(self.file, "\n## {}", path.display())?;
        writeln!(self.file, "\n```\n{}\n```", content.trim())?;

        Ok(())
    }
}
