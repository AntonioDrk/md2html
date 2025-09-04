# md2html — Markdown to HTML converter

A small CLI tool that converts a single Markdown file to HTML. It supports headers, paragraphs, bold/italic, unordered and ordered lists, code blocks (triple backticks) and some inline formatting. See the parser implementation in [`tokenize_text`](src/parser.rs) / [`tokenize_line`](src/parser.rs).

# Links
- Source: [src/parser.rs](src/parser.rs), [src/main.rs](src/main.rs)
- Sample input: [input/in.md](input/in.md)
- Example output: [src/Untitled-1.html](src/Untitled-1.html)
- Manifest: [Cargo.toml](Cargo.toml)

# Quick start (build & run)
1. Build:
   cargo build --release

2. Convert the default sample:
   cargo run --release

   By default the program reads input/in.md and writes output/out.html (created under the workspace root).

# CLI usage
- `--input <FILE>`    Absolute or relative path to the input Markdown file.
- `--output <DIR>`   Directory where out.html will be created (default: ./output).
- `--help`            Show help.
- `--version`         Show version.

## Example
- Convert a custom file and write to a specific folder (<b>Windows example</b>):
  `cargo run --release -- --input "C:\path\to\my.md" --output "C:\path\to\out_dir"`

- Convert the included sample and open result:</br>
  ```console
  > cargo run --release
  > start output\out.html
  ```

# Testing
- Run unit tests:
  `cargo test`

# ⚠️ Notes & limitations
- Inline code handling and some complex nesting are partially implemented; see tests in [src/parser.rs](src/parser.rs).
- Code blocks preserve raw lines inside <pre><code>...</code></pre>.
- If you want to extend parsing, edit [src/parser.rs](src/parser.rs) and add more inline/token rules.

# License

This project is provided under the MIT License — see the included LICENSE file for full terms.