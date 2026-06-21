mod algorithms;
mod contracts;
mod data_structure;
mod io;

use std::io::Read;
use std::process;

use clap::{ArgAction, CommandFactory, Parser as ClapParser};
use contracts::AppError;
use data_structure::DataStructure;

const AFTER_HELP: &str = "Examples:\n  cargo run -- tire --parser string-array --input '[\"data\", \"bus\", \"cat\", \"car\"]'\n  echo '[\"data\", \"bus\"]' | cargo run -- tire --parser string-array\n  echo 'hello world' | cargo run -- some-algorithm --parser raw-string";

#[derive(Debug, ClapParser)]
#[command(
    name = "algorithm-cheatsheet",
    version,
    about = "Run registered algorithms with optional input parsers.",
    long_about = "Run registered algorithms with optional input parsers.\n\nThe algorithm name is positional. The raw input comes from --input when provided, otherwise it is read from STDIN. A parser converts the raw string into a shared data structure before the selected algorithm runs.",
    after_help = AFTER_HELP
)]
struct Cli {
    #[arg(
        value_name = "ALGORITHM-NAME",
        help = "Algorithm to run from the algorithms module."
    )]
    algorithm_name: Option<String>,

    #[arg(
        long,
        value_name = "INPUT",
        help = "Raw input string. If omitted, the program reads from STDIN."
    )]
    input: Option<String>,

    #[arg(
        long,
        value_name = "PARSER-NAME",
        help = "Parser to use from the io module before running the algorithm."
    )]
    parser: Option<String>,

    #[arg(
        long = "list-algorithms",
        action = ArgAction::SetTrue,
        help = "List registered algorithms and exit."
    )]
    list_algorithms: bool,

    #[arg(
        long = "list-parsers",
        action = ArgAction::SetTrue,
        help = "List registered parsers and exit."
    )]
    list_parsers: bool,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let cli = Cli::parse();

    if cli.list_algorithms {
        print_algorithms();
    }

    if cli.list_parsers {
        print_parsers();
    }

    let Some(algorithm_name) = cli.algorithm_name else {
        if cli.list_algorithms || cli.list_parsers {
            return Ok(());
        }

        Cli::command().print_help()?;
        println!();
        return Ok(());
    };

    let algorithm = algorithms::create_algorithm(&algorithm_name)
        .ok_or_else(|| AppError::UnknownAlgorithm(algorithm_name.clone()))?;

    let raw_input = read_raw_input(cli.input)?;
    let structured_input = parse_input(cli.parser.as_deref(), &raw_input)?;
    let output = algorithm.run(&structured_input)?;

    println!("{output}");

    Ok(())
}

fn print_algorithms() {
    println!("Available algorithms:");

    for algorithm in algorithms::available_algorithms() {
        println!("- {}: {}", algorithm.name(), algorithm.description());
    }
}

fn print_parsers() {
    println!("Available parsers:");

    for parser in io::available_parsers() {
        println!("- {}: {}", parser.name(), parser.description());
    }
}

fn read_raw_input(input: Option<String>) -> Result<String, AppError> {
    match input {
        Some(raw) => Ok(raw),
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

fn parse_input(parser_name: Option<&str>, raw_input: &str) -> Result<DataStructure, AppError> {
    match parser_name {
        Some(name) => {
            let parser =
                io::create_parser(name).ok_or_else(|| AppError::UnknownParser(name.to_string()))?;
            Ok(parser.parse(raw_input)?)
        }
        None => Ok(DataStructure::RawString(raw_input.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_input, Cli};
    use crate::data_structure::DataStructure;
    use clap::Parser as ClapParser;

    #[test]
    fn parses_algorithm_and_parser_flags() {
        let cli = Cli::parse_from([
            "algorithm-cheatsheet",
            "tire",
            "--parser",
            "string-array",
            "--input",
            "[\"data\"]",
        ]);

        assert_eq!(cli.algorithm_name.as_deref(), Some("tire"));
        assert_eq!(cli.parser.as_deref(), Some("string-array"));
        assert_eq!(cli.input.as_deref(), Some("[\"data\"]"));
    }

    #[test]
    fn defaults_to_raw_string_when_no_parser_is_selected() {
        let parsed = parse_input(None, "plain text").expect("raw string should parse");

        assert_eq!(parsed, DataStructure::RawString("plain text".to_string()));
    }
}
