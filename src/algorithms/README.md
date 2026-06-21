# Algorithms Module

This directory contains runnable algorithm implementations.

- each algorithm lives in its own Rust module
- each implementation must satisfy the shared `Algorithm` trait
- register new algorithms in `mod.rs` so the CLI can find them by name
- accept parsed input through the shared `DataStructure` enum

The `tire` implementation is intentionally a stub so repository-specific logic can be filled in later.
