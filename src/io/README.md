# Input and Output Notes

This directory contains parser implementations that convert raw CLI or STDIN text into shared data structures.

- each parser must satisfy the shared `Parser` trait
- return values through the shared `DataStructure` enum
- register new parsers in `mod.rs` so the CLI can look them up by name
- keep parsing logic separate from algorithm execution logic
