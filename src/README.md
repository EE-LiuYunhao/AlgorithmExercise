# Source Layout

The `src` directory holds the command-line entry point, shared contracts, and runtime modules.

- `main.rs` parses CLI arguments with `clap`, reads raw input, and dispatches work
- `contracts.rs` defines the traits and shared error types used across the crate
- `data_structure/` defines the shared data enum exchanged between parsers and algorithms
- `algorithms/` keeps runnable algorithm implementations and their registry
- `parser/` keeps parser implementations and their registry

Add new modules here when the project needs more data types, algorithms, or parsers.
