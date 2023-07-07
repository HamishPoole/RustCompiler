#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::path::PathBuf;

use clap::{
    crate_version, Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches, Parser, Subcommand,
};
use log::error;
use regex::internal::Compiler;

use vc::parser::{parse_code, ParserData};
use vc::scanner::Scanner;
use vc::{parse_print_ast, parse_unparse, print_tokens, test_parser};

#[derive(Parser)]
#[clap(author = "Hamish Poole", about = "A compiler for the VC language.")]
#[command(version = crate_version!(), propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Scans the input file and outputs tokens found.
    #[command(arg_required_else_help = true)]
    Scan { input_filepath: String },

    /// Parses the input file and prints the AST produced.
    #[command(arg_required_else_help = true)]
    Parse { input_filepath: String },

    /// Parses the input file, then unparses the AST and prints an identical result.
    #[command(arg_required_else_help = true)]
    Unparse { input_filepath: String },
}

#[derive(Args)]
struct CommandArgs {}

fn main() {
    pretty_env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { input_filepath } => {
            print_tokens(&input_filepath);
        }
        Commands::Parse { input_filepath } => {
            parse_print_ast(&input_filepath);
        }
        Commands::Unparse { input_filepath } => {
            parse_unparse(&input_filepath);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
