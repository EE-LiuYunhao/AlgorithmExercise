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

pub fn available_parsers() -> [&'static dyn Parser; 2] {
    [&STRING_ARRAY_PARSER, &RAW_STRING_PARSER]
}

pub fn create_parser(name: &str) -> Option<&'static dyn Parser> {
    match name {
        "graph" => Some(&GRAPH_PARSER),
        "int-array" => Some(&INT_ARRAY_PARSER),
        "int" => Some(&INT_PARSER),
        "raw-string" => Some(&RAW_STRING_PARSER),
        "string-array" => Some(&STRING_ARRAY_PARSER),
        _ => None,
    }
}
