#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::path::PathBuf;

use clap::{Arg, ArgAction, ArgMatches, Args, Command, FromArgMatches, Parser, Subcommand};
use log::error;

use vc_compiler::parser::ParserData;
use vc_compiler::scanner::Scanner;
use vc_compiler::utils::print_stuff;

#[derive(Parser)]
#[command(author, version, about)]
struct Arguments {
    /// Filename to compile.
    #[arg(required = true, index = 1)]
    input_filepath: Option<String>,

    /// Output filepath.
    #[arg(required = true, index = 2)]
    output_filepath: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    print_stuff();
    let cli = Arguments::parse();

    let input_filepath = cli.input_filepath.as_deref().unwrap();
    let output_filepath = cli.output_filepath.as_deref().unwrap();

    // test_scanner(input_filepath, output_filepath);

    let contents_string = std::fs::read_to_string(input_filepath).expect("File reading error.");

    let mut scanner = Scanner::new(contents_string);
    let mut ast = ParserData::new(scanner);
}

fn generate_filetype(filename: &str) {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
