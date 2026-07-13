use crate::contracts::{ParseError, Parser};
use crate::data_structure::DataStructure;

pub(super) struct IntParser;

impl Parser for IntParser {
    fn name(&self) -> &'static str {
        "int"
    }

    fn description(&self) -> &'static str {
        "Parse the input into a single int"
    }

    fn parse(&self, raw_input: &str) -> Result<DataStructure, ParseError> {
        let raw_input = raw_input.trim();
        let val = raw_input.parse::<i32>();
        match val {
            Result::Ok(int_val) => return Ok(DataStructure::Int(int_val)),
            Result::Err(err) => {
                return Err(ParseError::new(format!(
                    "cannot parse {} into int, due to error: {}",
                    raw_input, err
                )))
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::IntParser;
    use crate::{contracts::Parser, data_structure::DataStructure};

    #[test]
    fn parses_plain_integer() {
        let parser = IntParser;

        let parsed = parser.parse("42").expect("plain int should parse");

        assert_eq!(parsed, DataStructure::Int(42));
    }

    #[test]
    fn parses_integer_with_surrounding_whitespace() {
        let parser = IntParser;

        let parsed = parser
            .parse(" 42\n")
            .expect("int with surrounding whitespace should parse");

        assert_eq!(parsed, DataStructure::Int(42));
    }
}
