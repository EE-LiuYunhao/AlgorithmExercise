mod graph;
mod int;
mod int_array;
mod raw_string;
mod string_array;

use crate::contracts::Parser;

static GRAPH_PARSER: graph::GraphParser = graph::GraphParser;
static INT_ARRAY_PARSER: int_array::IntArrayParser = int_array::IntArrayParser;
static INT_PARSER: int::IntParser = int::IntParser;
static RAW_STRING_PARSER: raw_string::RawStringParser = raw_string::RawStringParser;
static STRING_ARRAY_PARSER: string_array::StringArrayParser = string_array::StringArrayParser;

/// Returns all parsers registered in the CLI.
///
/// The parser order matters for auto-detection when no parser is explicitly
/// selected by the user.
pub fn available_parsers() -> [&'static dyn Parser; 5] {
    [
        &GRAPH_PARSER,
        &INT_ARRAY_PARSER,
        &INT_PARSER,
        &STRING_ARRAY_PARSER,
        &RAW_STRING_PARSER,
    ]
}

/// Creates a registered parser instance by its canonical name.
pub fn create_parser(name: &str) -> Option<&'static dyn Parser> {
    available_parsers()
        .into_iter()
        .find(|parser| parser.name() == name)
}
