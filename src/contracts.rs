use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use crate::data_structure::DataStructure;
use crate::debug::DebugPrinter;

pub trait Algorithm {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn run(
        &self,
        input: &DataStructure,
        debug: &DebugPrinter,
    ) -> Result<DataStructure, AlgorithmError>;
}

pub trait Parser {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn parse(&self, raw_input: &str) -> Result<DataStructure, ParseError>;
}

#[derive(Debug)]
pub enum AppError {
    UnknownAlgorithm(String),
    UnknownParser(String),
    Io(std::io::Error),
    Parse(ParseError),
    Algorithm(AlgorithmError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownAlgorithm(name) => write!(
                f,
                "unknown algorithm `{name}`. Run with --list-algorithms to see what is registered."
            ),
            Self::UnknownParser(name) => write!(
                f,
                "unknown parser `{name}`. Run with --list-parsers to see what is registered."
            ),
            Self::Io(error) => write!(f, "failed to read input: {error}"),
            Self::Parse(error) => Display::fmt(error, f),
            Self::Algorithm(error) => Display::fmt(error, f),
        }
    }
}

impl Error for AppError {}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::new(format!("cannot parse string to int_32, due to error: {:?}", value))
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ParseError> for AppError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<AlgorithmError> for AppError {
    fn from(value: AlgorithmError) -> Self {
        Self::Algorithm(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    #[allow(dead_code)]
    pub fn not_implemented(name: &str) -> Self {
        Self::new(format!("parser `{name}` is registered but not implemented yet"))
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "input parse error: {}", self.message)
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorithmError {
    message: String,
}

impl AlgorithmError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    #[allow(dead_code)]
    pub fn not_implemented(name: &str) -> Self {
        Self::new(format!("algorithm `{name}` is registered but not implemented yet"))
    }

    #[allow(dead_code)]
    pub fn invalid_input(expected: &str, actual: &DataStructure) -> Self {
        Self::new(format!(
            "algorithm expected {expected}, but received {}",
            actual.kind()
        ))
    }
}

impl Display for AlgorithmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "algorithm error: {}", self.message)
    }
}

impl Error for AlgorithmError {}

#[cfg(test)]
mod tests {
    use super::AlgorithmError;
    use crate::data_structure::DataStructure;

    #[test]
    fn invalid_input_mentions_expected_and_actual_types() {
        let error = AlgorithmError::invalid_input(
            "string-array",
            &DataStructure::RawString("hello".to_string()),
        );

        assert_eq!(
            error.to_string(),
            "algorithm error: algorithm expected string-array, but received raw-string"
        );
    }
}
