use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

use crate::data_structure::DataStructure;
use crate::debug::DebugPrinter;

/// Common interface for all registered algorithms.
///
/// Implementors expose a stable command-line name, a human-readable description,
/// and the execution entry point that consumes a parsed [`DataStructure`].
pub trait Algorithm {
    /// Returns the stable registry name used by the CLI.
    fn name(&self) -> &'static str;

    /// Returns a human-readable summary shown in help and listing output.
    fn description(&self) -> &'static str;

    /// Executes the algorithm against an already parsed input payload.
    ///
    /// Implementations should validate that `input` has the expected shape and
    /// return [`AlgorithmError`] when the payload kind is unsupported or the
    /// algorithm-specific execution fails.
    fn run(
        &self,
        input: &DataStructure,
        debug: &DebugPrinter,
    ) -> Result<DataStructure, AlgorithmError>;
}

/// Common interface for all registered input parsers.
///
/// Parsers convert raw user input into a shared [`DataStructure`] value so that
/// algorithms can operate on typed payloads instead of plain text.
pub trait Parser {
    /// Returns the stable registry name used by the CLI.
    fn name(&self) -> &'static str;

    /// Returns a human-readable summary shown in help and listing output.
    fn description(&self) -> &'static str;

    /// Parses a raw input string into a structured payload.
    fn parse(&self, raw_input: &str) -> Result<DataStructure, ParseError>;
}

/// Top-level application error for CLI execution.
#[derive(Debug)]
pub enum AppError {
    /// The user requested an algorithm name that is not registered.
    UnknownAlgorithm(String),

    /// The user requested a parser name that is not registered.
    UnknownParser(String),

    /// Reading input from stdin or other I/O sources failed.
    Io(std::io::Error),

    /// Converting raw input into a structured payload failed.
    Parse(ParseError),

    /// Running a selected algorithm failed.
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
        Self::new(format!(
            "cannot parse string to int_32, due to error: {:?}",
            value
        ))
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

/// Error returned when raw user input cannot be parsed into a [`DataStructure`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    /// Creates a new parse error from any string-like message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    #[allow(dead_code)]
    /// Creates a standard error message for a parser that is registered but not implemented.
    pub fn not_implemented(name: &str) -> Self {
        Self::new(format!(
            "parser `{name}` is registered but not implemented yet"
        ))
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "input parse error: {}", self.message)
    }
}

impl Error for ParseError {}

/// Error returned when an algorithm cannot be executed successfully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorithmError {
    message: String,
}

impl AlgorithmError {
    /// Creates a new algorithm error from any string-like message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    #[allow(dead_code)]
    /// Creates a standard error message for an algorithm that is registered but not implemented.
    pub fn not_implemented(name: &str) -> Self {
        Self::new(format!(
            "algorithm `{name}` is registered but not implemented yet"
        ))
    }

    #[allow(dead_code)]
    /// Creates an error describing a mismatch between expected and actual input kinds.
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
