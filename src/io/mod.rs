mod raw_string;
mod string_array;
mod graph;

use crate::contracts::Parser;

static RAW_STRING_PARSER: raw_string::RawStringParser = raw_string::RawStringParser;
static STRING_ARRAY_PARSER: string_array::StringArrayParser = string_array::StringArrayParser;
static GRAPH_PARSER: graph::GraphParser = graph::GraphParser;

pub fn available_parsers() -> [&'static dyn Parser; 2] {
    [&STRING_ARRAY_PARSER, &RAW_STRING_PARSER]
}

pub fn create_parser(name: &str) -> Option<&'static dyn Parser> {
    match name {
        "raw-string" => Some(&RAW_STRING_PARSER),
        "string-array" => Some(&STRING_ARRAY_PARSER),
        "graph" => Some(&GRAPH_PARSER),
        _ => None,
    }
}
