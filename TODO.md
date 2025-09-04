# Progress Checklist

## 🧱 Core Features

- [x] **Project Setup**
  - [x] Create project with `cargo new markdown_converter`
  - [x] Organize files into `main.rs`, `parser.rs`, and optionally `html.rs`

- [x] **Input Handling**
  - [x] Read from a file (e.g., `input.md`)
  - [ ] (Optional) Read from `stdin`

- [ ] **Markdown Parsing**
  - [x] Parse headers (`#`, `##`, etc.)
  - [x] Parse paragraphs
  - [x] Parse **bold** (`**text**`)
  - [x] Parse *italic* (`*text*`)
  - [x] Parse unordered lists (`- item`)
  - [x] Parse ordered lists (`1. item`)
  - [ ] Parse inline code (`` `code` ``)
  - [x] Parse code blocks (``` triple backticks ```)

- [x] **HTML Output**
  - [x] Convert parsed Markdown to HTML
  - [x] Output to terminal or save to file

- [x] **CLI Interface**
  - [x] Add basic CLI options (e.g., `--input`, `--output`)
  - [x] Add `--help` and `--version` flags

---

## 🌟 Bonus Features

- [ ] **Live Terminal Preview**
  - [ ] Integrate `termimad`
  - [ ] Display a live-formatted Markdown preview in terminal

- [ ] **File Watching**
  - [ ] Use `notify` to watch file changes
  - [ ] Re-parse and re-render when file is modified

---

## 🧪 Testing & Debugging

- [x] **Basic Unit Tests**
  - [x] Test header parsing
  - [x] Test inline formatting
  - [ ] Test HTML output generation

- [ ] **Debugging Tips**
  - [ ] Use `dbg!()` macro to inspect variables
  - [ ] Run with `RUST_BACKTRACE=1` to see full stack traces
  - [ ] Use `cargo check` to catch issues without building
  - [ ] Use `cargo clippy` for linting and code suggestions