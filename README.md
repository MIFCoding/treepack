# Treepack

![Rust](https://img.shields.io/badge/Rust-2024-dea584?logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue)
[![lang ru](https://img.shields.io/badge/lang-ru-green)](README.ru.md)

A command-line tool that recursively collects UTF-8 text files into a single Markdown document, with file paths as headings and contents in code blocks.

## Installation

Install [Rust and Cargo](https://www.rust-lang.org/tools/install), then run from the project directory:

```sh
cargo install --path . --locked
```

## Usage

Pack the current directory into `treepack.md`:

```sh
treepack
```

Choose an input directory and output file:

```sh
treepack -d ./src -o sources.md
```

| Option | Description | Default |
| --- | --- | --- |
| `-d, --directory` | Directory to scan | `.` |
| `-o, --output` | Output file; adds `.md` if no extension is provided | `treepack.md` |
| `-h, --help` | Show help | |
| `-V, --version` | Show version | |

The output file is overwritten and excluded from the scan. Unreadable and non-text files are skipped. Hidden files and directories are included; `.gitignore` rules are not applied.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
