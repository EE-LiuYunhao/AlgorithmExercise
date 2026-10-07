# Algorithm Cheatsheet

A Rust command-line application for running registered algorithms against typed input produced by reusable parsers.

## Quick start

List the names currently available to the CLI:

```bash
cargo run -- --list-algorithms
cargo run -- --list-parsers
```

Run an algorithm with inline input:

```bash
cargo run -- longest-consecutive-sequence \
  --parser int-array \
  --input '[100, 4, 200, 1, 3, 2]'
# 4
```

Input can also be read from standard input:

```bash
echo '[A, B, C] [(A, B, 2), (B, C, 3), (A, C, 10)]' \
  | cargo run -- dijkstra --parser graph
# ["A -> B", "A -> B -> C"]
```

Pass `--verbose` (or `-v`) to print algorithm debug messages. If `--parser` is omitted, the program tries the registered parsers in registry order; `raw-string` is the final fallback and accepts any input.

## CLI contract

```text
cargo run -- [ALGORITHM-NAME] [--parser PARSER-NAME] [--input INPUT] [--verbose]
```

- `ALGORITHM-NAME` selects an entry from the algorithm registry.
- `--parser` selects the parser that converts text into a shared `DataStructure` value.
- `--input` supplies text directly. Without it, the CLI reads all input from STDIN.
- `--list-algorithms` and `--list-parsers` print the corresponding registries. They can be used without an algorithm name.
- The selected algorithm's result is printed to STDOUT; user-facing failures are printed to STDERR and exit with status 1.

## Repository architecture

```text
src/
├── main.rs                         CLI definition, input handling, parser selection, and dispatch
├── contracts.rs                    Algorithm/Parser traits and shared error types
├── debug.rs                        --verbose debug-output helper
├── data_structure/
│   ├── mod.rs                      DataStructure enum and output formatting
│   ├── graph.rs                    directed weighted-graph payload
│   └── linked_list.rs              linked-list payload and helpers
├── algorithms/
│   ├── mod.rs                      algorithm registry
│   ├── dijkstra.rs
│   ├── longest_consecutive_sequence.rs
│   └── tire.rs
└── parser/
    ├── mod.rs                      parser registry and auto-detection order
    ├── graph.rs
    ├── int.rs
    ├── int_array.rs
    ├── raw_string.rs
    └── string_array.rs
```

The runtime flow is:

```text
CLI/STDIN text -> selected (or auto-detected) Parser -> DataStructure -> Algorithm -> DataStructure -> STDOUT
```

Algorithms and parsers are discovered through their registries, not automatically from filenames. A new implementation must satisfy the relevant trait in `contracts.rs` and be added to `algorithms/mod.rs` or `parser/mod.rs`.

## Registered algorithms

| CLI name | Expected input | Behavior |
| --- | --- | --- |
| `dijkstra` | `graph` | Returns shortest paths from the first declared node to every later declared node. Edge costs must be non-negative. |
| `longest-consecutive-sequence` | `int-array` | Returns the length of the longest consecutive-value sequence. |
| `tire` | `string-array` | Builds the repository's prefix-tree encoding and returns one encoded prefix per input string. |

## Registered parsers

| CLI name | Input example | Produced value |
| --- | --- | --- |
| `graph` | `[A, B] [(A, B, 5)]` | Directed graph; the edge cost is optional and defaults to `0`. |
| `int-array` | `[1, 2, 3]` | Array of non-negative 32-bit integers. |
| `int` | `42` | One signed 32-bit integer. |
| `string-array` | `["data", "bus"]` | Array of double-quoted strings, with common escapes supported. |
| `raw-string` | `hello world` | The original text without conversion. |

## Implementation status and TODOs

All registered algorithms and parsers currently have implementations. In particular, `tire` and `string-array` are no longer placeholders, and there are no implementation TODO markers in the source tree.

The remaining limitations worth addressing are:

- `int-array` does not currently accept negative integers, although the single `int` parser does.
- Parser auto-detection is order-dependent. Ambiguous input uses the first parser that succeeds, while `raw-string` guarantees a final fallback.
- The algorithm registry exposes `tire`, although the conventional spelling of the data structure is `trie`; changing it would affect the CLI name.

## Development

Run the test suite with:

```bash
cargo test
```
