# Algorithm Cheatsheet

Rust command-line project for running named algorithm implementations with pluggable input parsers.

## Purpose

- dispatch algorithms by name with `cargo run -- [ALGORITHM-NAME]`
- parse raw string input through reusable parser implementations in `src/io`
- share input data definitions through `src/data_structure`
- keep algorithm contracts, parser contracts, and runtime wiring explicit and easy to extend

## Structure

- `src/main.rs` owns the `clap` CLI and the runtime dispatch flow
- `src/contracts.rs` defines the shared `Algorithm` and `Parser` traits plus error types
- `src/data_structure` defines the shared data enum passed from parsers to algorithms
- `src/algorithms` contains registered algorithm implementations
- `src/io` contains registered parser implementations

## Quick start

```bash
cargo run -- --list-algorithms
cargo run -- --list-parsers
cargo run -- tire --parser string-array --input '["data", "bus", "cat", "car"]'
cargo run -- --verbose tire --parser string-array --input '["data", "bus"]'
echo '["data", "bus", "cat", "car"]' | cargo run -- tire --parser string-array
```

## Current placeholders

- `tire` is registered but intentionally left unimplemented
- `string-array` is registered but intentionally left unimplemented
- `raw-string` is available as a simple parser that passes the input through unchanged
