# Text Inspector – Chapter 1 Practice Task

This CLI tool analyzes a text file and prints insights about its content:  
line count, word count, unique words, and filtering by keywords.

>This task is inspired by the content of **Chapter 1** of the [Ultimate Rust Foundation Course](https://www.ardanlabs.com/training/individual-on-demand/rust-bundle/) and the official code [GitHub Repo](https://github.com/thebracket/ArdanUltimateRust-5Days).  
>It is **independently designed** to deepen understanding and apply course concepts to a real task.

---

## Features

- CLI input via `clap`
- File loading abstraction via trait
- Clean Architecture: Core, Services, Infrastructure, CLI
- Fully testable logic
- Optional word filtering

---

## Run

```bash
cargo run -- --file file.txt --filter rust
