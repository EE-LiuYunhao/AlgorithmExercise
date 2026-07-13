# Repository Agent Notes

## Purpose

This repository is a Rust CLI for running algorithm implementations by name with optional parser-based input conversion.

## Architecture Rules

- keep CLI wiring in `src/main.rs`
- keep shared traits and cross-cutting errors in `src/contracts.rs`
- keep shared algorithm/parser payloads in `src/data_structure`
- keep algorithm implementations in `src/algorithms`
- keep parser implementations in `src/parser`

## Runtime Contract

- `cargo run -- [ALGORITHM-NAME]` selects an algorithm from `src/algorithms`
- `cargo run -- [ALGORITHM-NAME] --input "..."` uses the provided raw input string
- when `--input` is omitted, raw input must be read from STDIN
- `cargo run -- [ALGORITHM-NAME] --parser [PARSER-NAME]` selects a parser from `src/parser`
- parsers convert raw text into a shared `DataStructure` value before algorithm execution
- algorithms and parsers must be registered in their module registries so the CLI can find them

## Caveats

- use `clap` for CLI definitions and help text
- keep `tire` intentionally unimplemented until repository-specific logic is added
- keep `string-array` intentionally unimplemented until repository-specific parsing logic is added
- update `README.md` when CLI behavior, module layout, or registered names change
