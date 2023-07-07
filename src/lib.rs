#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::fs::File;
use std::io;
use std::io::Write;

use crate::ast::{AstNode, PrintAST};
use crate::parser::{parse_program, ParserData};
use crate::scanner::Scanner;

mod ast;
pub mod checker;
mod globals;
pub mod parser;
pub mod scanner;
pub mod stages;
pub mod token;
pub mod utils;

pub fn print_tokens(input_filepath: &str) {
    let input_filestring = std::fs::read_to_string(input_filepath).expect("File reading error.");

    let mut my_scanner = Scanner::new(input_filestring);
    // Passing clone creates local data in the object.
    // Write a loop that prints all the tokens of the Scanner object, until a TokenKind::EOF is returned.

    // let mut out_file = File::create(output_filepath).expect("Expected to create file.");

    loop {
        let mut token = my_scanner.get_next_token();
        let token_string = format!("{:?}\n", token);

        println!("{}", token_string);

        if token.token_kind == token::TokenKind::EOF || token.token_kind == token::TokenKind::ERROR
        {
            break;
        }
    }
}

pub fn parse_print_ast(input_filepath: &str) {
    let contents_string = std::fs::read_to_string(input_filepath).expect("File reading error.");

    let scanner = Scanner::new(contents_string);

    let program = parse_program(scanner);

    program.print_program();
}

pub fn parse_unparse(input_filepath: &str) {
    let contents_string = std::fs::read_to_string(input_filepath).expect("File reading error.");

    let scanner = Scanner::new(contents_string);

    let program = parse_program(scanner);

    program.print_unparsed_program();
}

pub fn test_parser(input_filepath: &str) {
    let contents_string = std::fs::read_to_string(input_filepath).expect("File reading error.");

    let scanner = Scanner::new(contents_string);

    // Passing clone creates local data in the object.
    // Write a loop that prints all the tokens of the Scanner object, until a TokenKind::EOF is returned.

    let program = parse_program(scanner);
    println!("{:?}", program);

    program.print_program();
    program.print_unparsed_program();

    // For each layer of the AST, tab the node name in by depth + tab size.
    // Then, print the entire AST.
}