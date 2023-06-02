#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches, Parser, Subcommand};
use log::error;
use regex::internal::Compiler;

use vc::{test_parser, test_scanner};
use vc::parser::{parse_program, ParserData};
use vc::scanner::Scanner;

#[derive(Parser)]
#[clap(author = "Hamish Poole", about = "A compiler for the VC language.")]
#[command(propagate_version = true)]
struct Cli {
    /// Filename to compile
    input_filepath: Option<String>,

    /// Output filepath.
    output_filepath: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scans the input file and outputs tokens found.
    Scan,

    /// Parses the input file and prints the AST produced.
    Parse,
}

#[derive(Args)]
struct CommandArgs {}

fn main() {
    let cli = Cli::parse();

    let input_filepath = cli.input_filepath.as_deref().unwrap();
    let output_filepath = cli.output_filepath.as_deref().unwrap();

    match &cli.command {
        Commands::Scan => {
            test_scanner(input_filepath, output_filepath);
        }
        Commands::Parse=> {
            test_parser(input_filepath, output_filepath);
        }
    }

    // let mut scanner = Scanner::new(contents_string);
    // let mut ast = parse_program(scanner);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
