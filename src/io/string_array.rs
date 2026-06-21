use crate::contracts::{ParseError, Parser};
use crate::data_structure::DataStructure;

pub struct StringArrayParser;

impl Parser for StringArrayParser {
    fn name(&self) -> &'static str {
        "string-array"
    }

    fn description(&self) -> &'static str {
        "Placeholder parser for turning a raw string into Vec<String>."
    }

    fn parse(&self, _raw_input: &str) -> Result<DataStructure, ParseError> {
        Err(ParseError::not_implemented(self.name()))
    }
}
