#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::Write;

use crate::scanner::Scanner;

mod ast;
pub mod checker;
mod globals;
pub mod parser;
pub mod scanner;
pub mod stages;
pub mod token;
pub mod utils;

pub fn test_scanner(input_filepath: &str, output_filepath: &str) {
    let input_filestring = std::fs::read_to_string(input_filepath).expect("File reading error.");

    let mut my_scanner = Scanner::new(input_filestring);
    // Passing clone creates local data in the object.
    // Write a loop that prints all the tokens of the Scanner object, until a TokenKind::EOF is returned.

    let mut out_file = File::create(output_filepath).expect("Expected to create file.");

    loop {
        let mut token = my_scanner.get_next_token();
        let token_string = format!("{:?}\n", token);

        out_file
            .write_all(token_string.as_bytes())
            .expect("Expected to write to file.");

        if token.token_kind == token::TokenKind::EOF || token.token_kind == token::TokenKind::ERROR
        {
            break;
        }
    }
}

pub fn test_parser(input_filepath: &str, output_filepath: &str) {
    let input_filestring = std::fs::read_to_string(input_filepath).expect("File reading error.");

    let mut my_scanner = Scanner::new(input_filestring);
    // Passing clone creates local data in the object.
    // Write a loop that prints all the tokens of the Scanner object, until a TokenKind::EOF is returned.

    let mut out_file = File::create(output_filepath).expect("Expected to create file.");

    loop {
        let mut token = my_scanner.get_next_token();
        let token_string = format!("{:?}\n", token);

        out_file
            .write_all(token_string.as_bytes())
            .expect("Expected to write to file.");

        if token.token_kind == token::TokenKind::EOF || token.token_kind == token::TokenKind::ERROR
        {
            break;
        }
    }
}