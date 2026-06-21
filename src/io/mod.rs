mod raw_string;
mod string_array;

use crate::contracts::Parser;

pub fn available_parsers() -> Vec<Box<dyn Parser>> {
    vec![
        Box::new(raw_string::RawStringParser),
        Box::new(string_array::StringArrayParser),
    ]
}

pub fn create_parser(name: &str) -> Option<Box<dyn Parser>> {
    match name {
        "raw-string" => Some(Box::new(raw_string::RawStringParser)),
        "string-array" => Some(Box::new(string_array::StringArrayParser)),
        _ => None,
    }
}
