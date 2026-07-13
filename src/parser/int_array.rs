use std::num::ParseIntError;

use crate::{
    contracts::{ParseError, Parser},
    data_structure::DataStructure,
};

pub(super) struct IntArrayParser;

impl Parser for IntArrayParser {
    fn name(&self) -> &'static str {
        "int-array"
    }

    fn description(&self) -> &'static str {
        "Parse an int array in format of [1, 2, 3, ...] to Vec<i32>"
    }

    fn parse(
        &self,
        raw_input: &str,
    ) -> Result<crate::data_structure::DataStructure, crate::contracts::ParseError> {
        let raw_input = raw_input.trim();

        if !raw_input.starts_with('[') || !raw_input.ends_with(']') {
            return Err(ParseError::new("input must be in format of [1, 2, 3, ...]"));
        }

        let inner = &raw_input[1..raw_input.len() - 1];
        if inner.trim().is_empty() {
            return Ok(DataStructure::IntArray(vec![]));
        }
        let mut scanner = inner.chars();
        let mut buffer = Vec::<char>::new();
        let mut output = Vec::<i32>::new();

        let add_i32 = |output_buffer: &mut Vec<i32>, char_buffer: &mut Vec<char>| {
            output_buffer.push(char_buffer.iter().collect::<String>().parse::<i32>()?);
            char_buffer.clear();
            Ok::<(), ParseIntError>(())
        };

        loop {
            let nullable_ch = scanner.next();
            match nullable_ch {
                None => {
                    add_i32(&mut output, &mut buffer)?;
                    break;
                }
                Some(c) => match c {
                    ',' => add_i32(&mut output, &mut buffer)?,
                    '0'..='9' => buffer.push(c),
                    c if c.is_whitespace() => {
                        if buffer.is_empty() {
                            continue;
                        } else {
                            return Err(ParseError::new(
                                "unexpected whitespace character. Only digit character is allowed here."
                            ));
                        }
                    }
                    _ => {
                        return Err(ParseError::new(format!(
                            "unexpected character {}. Only comma, whitespaces, or digit character is allowed here.",
                            c
                        )));
                    }
                },
            }
        }

        Ok(DataStructure::IntArray(output))
    }
}

#[cfg(test)]
mod tests {
    use super::IntArrayParser;
    use crate::{contracts::Parser, data_structure::DataStructure};

    #[test]
    fn parses_simple_int_array() {
        let parser = IntArrayParser;

        let parsed = parser.parse("[1, 2, 3]").expect("int array should parse");

        assert_eq!(parsed, DataStructure::IntArray(vec![1, 2, 3]));
    }

    #[test]
    fn parses_empty_int_array() {
        let parser = IntArrayParser;

        let parsed = parser.parse("[]").expect("empty int array should parse");

        assert_eq!(parsed, DataStructure::IntArray(vec![]));
    }

    #[test]
    fn rejects_invalid_character() {
        let parser = IntArrayParser;

        let error = parser
            .parse("[1, a]")
            .expect_err("non-digit item should be rejected");

        assert_eq!(
            error.to_string(),
            "input parse error: unexpected character a. Only comma, whitespaces, or digit character is allowed here."
        );
    }
}
