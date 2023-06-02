#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]


use std::fs::File;

use vc::scanner::Scanner;
use vc::test_parser;

#[test]
fn test_parser_fibonacci() {
    let input_filepath = "./tests/Parser/input/t57.vc";
    let output_filepath = "./tests/Parser/output/t57.vc";
    let solution_filepath = "./tests/Parser/solution/t57.sol";


    let mut my_scanner = Scanner::new(std::fs::read_to_string(input_filepath).expect("File reading error."));
}

#[test]
fn test_parser_defaults() {
    let input_filepath = "./defaults/test1.vc";
    let output_filepath = "./defaults/outfile1";


    let result_ast = test_parser(input_filepath, output_filepath);


    // TODO: have AST node print the parse tree.
    // TODO:
}
