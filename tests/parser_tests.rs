#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::fs::{read_to_string, File};
use std::process::Command;

use vc::parser::parse_code;
use vc::scanner::Scanner;
use vc::test_parser;

#[test]
fn test_parser_fibonacci() {
    let input_filepath = "./tests/Parser/input/t57.vc";

    let mut my_scanner =
        Scanner::new(std::fs::read_to_string(input_filepath).expect("File reading error."));
    parse_code(my_scanner);
}

#[test]
fn test_parser_numerics() {
    let input_filepath = "./tests/Parser/input/tNumerics.vc";

    let mut my_scanner =
        Scanner::new(std::fs::read_to_string(input_filepath).expect("File reading error."));
    parse_code(my_scanner);
}

#[test]
fn test_parser_bubble_sort() {
    let input_filepath = "./tests/Parser/input/t59.vc";

    let mut my_scanner =
        Scanner::new(std::fs::read_to_string(input_filepath).expect("File reading error."));
    parse_code(my_scanner);
}

#[test]
fn test_parser_defaults() {
    let input_filepath = "./defaults/test1.vc";
    let output_filepath = "./defaults/outfile1";

    let subprocess_output = Command::new("./target/debug/vc")
        .arg(input_filepath)
        .output()
        .expect("Failed to execute command.");

    let input_file_string = read_to_string(input_filepath).expect("File reading error.");
}
