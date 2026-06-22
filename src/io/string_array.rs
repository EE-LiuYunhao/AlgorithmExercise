use crate::contracts::{ParseError, Parser};
use crate::data_structure::DataStructure;

pub struct StringArrayParser;

impl Parser for StringArrayParser {
    fn name(&self) -> &'static str {
        "string-array"
    }

    fn description(&self) -> &'static str {
        "Parse a raw string in format of [\"a\", \"b\", \"c\", ...] to Vec<String>."
    }

    fn parse(&self, raw_input: &str) -> Result<DataStructure, ParseError> {
        let raw_input = raw_input.trim();

        if !raw_input.starts_with('[') || !raw_input.ends_with(']') {
            return Err(ParseError::new("input must be in format of [\"a\", \"b\", \"c\", ...]"));
        }

        let inner = &raw_input[1..raw_input.len() - 1];
        let mut chars = inner.chars().peekable();
        let mut output = Vec::<String>::new();

        skip_whitespace(&mut chars);
        if chars.peek().is_none() {
            return Ok(DataStructure::StringArray(output));
        }

        loop {
            skip_whitespace(&mut chars);

            if chars.next() != Some('"') {
                return Err(ParseError::new(
                    "each array item must be a double-quoted string",
                ));
            }

            let mut current = String::new();

            loop {
                match chars.next() {
                    Some('"') => break,
                    Some('\\') => {
                        let escaped = match chars.next() {
                            Some('"') => '"',
                            Some('\\') => '\\',
                            Some('n') => '\n',
                            Some('r') => '\r',
                            Some('t') => '\t',
                            Some(other) => {
                                return Err(ParseError::new(format!(
                                    "unsupported escape sequence: \\{other}"
                                )))
                            }
                            None => {
                                return Err(ParseError::new(
                                    "unterminated escape sequence inside string item",
                                ))
                            }
                        };
                        current.push(escaped);
                    }
                    Some(ch) => current.push(ch),
                    None => return Err(ParseError::new("unterminated string item")),
                }
            }

            output.push(current);
            skip_whitespace(&mut chars);

            match chars.next() {
                Some(',') => {
                    skip_whitespace(&mut chars);
                    if chars.peek().is_none() {
                        return Err(ParseError::new(
                            "trailing comma is not allowed in string-array input",
                        ));
                    }
                }
                None => break,
                Some(other) => {
                    return Err(ParseError::new(format!(
                        "unexpected character `{other}` after array item"
                    )))
                }
            }
        }

        Ok(DataStructure::StringArray(output))
    }
}

fn skip_whitespace<I>(chars: &mut std::iter::Peekable<I>)
where
    I: Iterator<Item = char>,
{
    while matches!(chars.peek(), Some(ch) if ch.is_whitespace()) {
        chars.next();
    }
}

#[cfg(test)]
mod tests {
    use super::StringArrayParser;
    use crate::contracts::Parser;
    use crate::data_structure::DataStructure;

    #[test]
    fn parses_simple_string_array() {
        let parser = StringArrayParser;

        let parsed = parser
            .parse(r#"["data", "bus", "cat", "car"]"#)
            .expect("string array should parse");

        assert_eq!(
            parsed,
            DataStructure::StringArray(vec![
                "data".to_string(),
                "bus".to_string(),
                "cat".to_string(),
                "car".to_string()
            ])
        );
    }

    #[test]
    fn parses_empty_string_array() {
        let parser = StringArrayParser;

        let parsed = parser.parse("[]").expect("empty array should parse");

        assert_eq!(parsed, DataStructure::StringArray(vec![]));
    }

    #[test]
    fn parses_escaped_characters() {
        let parser = StringArrayParser;

        let parsed = parser
            .parse(r#"["a\"b", "c\\d", "line\nbreak"]"#)
            .expect("escaped values should parse");

        assert_eq!(
            parsed,
            DataStructure::StringArray(vec![
                "a\"b".to_string(),
                "c\\d".to_string(),
                "line\nbreak".to_string()
            ])
        );
    }

    #[test]
    fn rejects_unquoted_items() {
        let parser = StringArrayParser;

        let error = parser.parse("[a]").expect_err("unquoted item should fail");

        assert_eq!(
            error.to_string(),
            "input parse error: each array item must be a double-quoted string"
        );
    }
}
