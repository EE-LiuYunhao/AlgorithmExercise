use crate::contracts::{ParseError, Parser};
use crate::data_structure::DataStructure;

pub struct RawStringParser;

impl Parser for RawStringParser {
    fn name(&self) -> &'static str {
        "raw-string"
    }

    fn description(&self) -> &'static str {
        "Wrap the raw input string without additional parsing."
    }

    fn parse(&self, raw_input: &str) -> Result<DataStructure, ParseError> {
        Ok(DataStructure::RawString(raw_input.to_string()))
    }
}
