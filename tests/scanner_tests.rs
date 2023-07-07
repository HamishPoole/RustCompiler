#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::fs::{File, read_to_string};
use std::io::Write;

use log::{debug, info, warn};
use test_log::test;

use vc::scanner;
use vc::scanner::Scanner;
use vc::token::{Token, TokenKind};
use vc::utils::SourcePosition;

fn scan_filepath(input_filepath: &str) -> Vec<Token> {
    let input_filestring = read_to_string(input_filepath).expect("File reading error.");

    let mut my_scanner = Scanner::new(input_filestring);

    let mut token_vector: Vec<Token> = Vec::new();

    loop {
        let curr_token = my_scanner.get_next_token();
        let curr_token_kind = curr_token.token_kind;
        token_vector.push(curr_token);

        if curr_token_kind == TokenKind::EOF || curr_token_kind == TokenKind::ERROR
        { break; }
    }

    token_vector
}

#[test]
fn test_string_scanning() {
    let mut token_vector: Vec<Token> = scan_filepath("./tests/scanner/inputFiles/string.vc");

    log::error!("{:?}", token_vector);

    for token in token_vector.iter() {
        log::debug!("Token: {:?}", token);
    }

    let expected_result = vec!(
        Token::new(TokenKind::STRINGLITERAL,
                   "comp3131".to_string(),
                   SourcePosition::new(3, 3, 3, 10)),
        Token::new(TokenKind::STRINGLITERAL,
                   "x\\ty".to_string(),
                   SourcePosition::new(4, 4, 3, 6)),
        Token::new(TokenKind::STRINGLITERAL,
                   "carriage return\\r".to_string(),
                   SourcePosition::new(5, 5, 3, 19)),
        Token::new(TokenKind::STRINGLITERAL,
                   "a\\'\\\\b".to_string(),
                   SourcePosition::new(6, 6, 3, 8)),
        Token::new(TokenKind::STRINGLITERAL,
                   "comp3131\\ncomp9102".to_string(),
                   SourcePosition::new(7, 7, 3, 20)),
        Token::new(TokenKind::EOF,
                   "$".to_string(),
                   SourcePosition::new(8, 8, 1, 1)),
    );

    assert_eq!(token_vector, expected_result);
}

#[test]
fn test_seps_newline() {
    let mut token_vector: Vec<TokenKind> = Vec::new();

    let filepath = "./tests/scanner/inputFiles/lparen_test.vc";
    let file_contents = std::fs::read_to_string(filepath).expect("File reading error.");

    let mut my_scanner = Scanner::new(file_contents);

    // Loop through scanner until EOF or error.  Add to token_vector on each loop.
    loop {
        let mut curr_token = my_scanner.get_next_token();
        token_vector.push(curr_token.token_kind);

        if curr_token.token_kind == TokenKind::EOF || curr_token.token_kind == TokenKind::ERROR
        { break; }
    }

    log::debug!("{:?}", token_vector);
    assert_eq!(
        token_vector,
        vec!(
            TokenKind::LPAREN,
            TokenKind::LPAREN,
            TokenKind::LPAREN,
            TokenKind::EOF
        )
    );
}

#[test]
fn test_all_separators() {
    let mut token_vector: Vec<TokenKind> = Vec::new();

    let filepath = "./tests/scanner/inputFiles/all_separators.vc";
    let file_contents = std::fs::read_to_string(filepath).expect("File reading error.");

    let mut my_scanner = Scanner::new(file_contents);

    // Loop through scanner until EOF or error.  Add to token_vector on each loop.
    loop {
        let curr_token = my_scanner.get_next_token();
        token_vector.push(curr_token.token_kind);

        log::debug!("{:?}", curr_token.token_kind);

        if curr_token.token_kind == TokenKind::EOF
            || curr_token.token_kind == TokenKind::ERROR
            || token_vector.len() > 100
        {
            break;
        }
    }

    log::debug!("{:?}", token_vector);
    assert_eq!(
        token_vector,
        vec!(
            TokenKind::LPAREN,
            TokenKind::RPAREN,
            TokenKind::LBRACE,
            TokenKind::RBRACE,
            TokenKind::LBRACKET,
            TokenKind::RBRACKET,
            TokenKind::SEMICOLON,
            TokenKind::COMMA,
            TokenKind::EOF
        )
    );
}

#[test]
fn test_literals_identifiers() {
    let mut token_vector: Vec<TokenKind> = Vec::new();

    let filepath = "./tests/scanner/inputFiles/literals_identifiers.vc";
    let file_contents = std::fs::read_to_string(filepath).expect("File reading error.");

    let mut my_scanner = Scanner::new(file_contents);

    // Loop through scanner until EOF or error.  Add to token_vector on each loop.
    loop {
        let curr_token = my_scanner.get_next_token();
        token_vector.push(curr_token.token_kind);

        log::debug!("{:?}", curr_token.token_kind);

        if curr_token.token_kind == TokenKind::EOF
            || curr_token.token_kind == TokenKind::ERROR
            || token_vector.len() > 100
        {
            break;
        }
    }

    log::debug!("{:?}", token_vector);
    assert_eq!(
        token_vector,
        vec!(
            TokenKind::INT,
            TokenKind::ID,
            TokenKind::SEMICOLON,
            TokenKind::FLOAT,
            TokenKind::ID,
            TokenKind::SEMICOLON,
            TokenKind::BOOLEAN,
            TokenKind::ID,
            TokenKind::SEMICOLON,
            TokenKind::EOF,
        )
    );
}

#[test]
fn test_gcd_signature() {
    let mut token_vector: Vec<TokenKind> = Vec::new();

    let filepath = "./tests/scanner/inputFiles/simple_gcd_signature.vc";
    let file_contents = std::fs::read_to_string(filepath).expect("File reading error.");

    let mut my_scanner = Scanner::new(file_contents);

    // Loop through scanner until EOF or error.  Add to token_vector on each loop.
    loop {
        let curr_token = my_scanner.get_next_token();
        token_vector.push(curr_token.token_kind);

        log::debug!("{:?}", curr_token.token_kind);

        if curr_token.token_kind == TokenKind::EOF
            || curr_token.token_kind == TokenKind::ERROR
            || token_vector.len() > 100
        {
            break;
        }
    }

    log::debug!("{:?}", token_vector);
    assert_eq!(
        token_vector,
        vec!(
            TokenKind::INT,
            TokenKind::ID,
            TokenKind::LPAREN,
            TokenKind::INT,
            TokenKind::ID,
            TokenKind::COMMA,
            TokenKind::INT,
            TokenKind::ID,
            TokenKind::RPAREN,
            TokenKind::LBRACE,
            TokenKind::RBRACE,
            TokenKind::EOF,
        )
    );
}

#[test]
fn test_gcd_whole_file() {
    let mut token_vector: Vec<TokenKind> = Vec::new();
    let input_filepath = "./tests/Scanner/inputFiles/gcd.vc";
    let output_filepath = "./tests/Scanner/outputFiles/gcd.sol";
    let solution_filepath = "./tests/Scanner/solutionFilesRust/gcd.sol";

    let mut my_scanner =
        Scanner::new(std::fs::read_to_string(input_filepath).expect("File reading error."));

    let mut out_file = File::create(output_filepath).expect("Expected to create file.");

    loop {
        let curr_token = my_scanner.get_next_token();
        token_vector.push(curr_token.token_kind);

        log::debug!("{:?}", curr_token.token_kind);
        let token_string = format!("{:?}\n", curr_token);

        out_file
            .write_all(token_string.as_bytes())
            .expect("Expected to write to file.");

        if curr_token.token_kind == TokenKind::EOF
            || curr_token.token_kind == TokenKind::ERROR

        {
            break;
        }
    }

    // Then, assert that the output file is the same as the solution file.
    let output = std::fs::read_to_string(output_filepath).expect("Unable to read output file");
    let solution =
        std::fs::read_to_string(solution_filepath).expect("Unable to read solution file");
    assert_eq!(output, solution, "Output file differs from solution file");
}

#[test]
fn test_fib_whole_file() {
    let mut token_vector: Vec<TokenKind> = Vec::new();
    let input_filepath = "./tests/Scanner/inputFiles/fib.vc";
    let output_filepath = "./tests/Scanner/outputFiles/fib.sol";
    let solution_filepath = "./tests/Scanner/solutionFilesRust/fib.sol";

    let mut my_scanner =
        Scanner::new(std::fs::read_to_string(input_filepath).expect("File reading error."));

    let mut out_file = File::create(output_filepath).expect("Expected to create file.");

    loop {
        let curr_token = my_scanner.get_next_token();
        token_vector.push(curr_token.token_kind);

        log::debug!("{:?}", curr_token.token_kind);
        let token_string = format!("{:?}\n", curr_token);

        out_file
            .write_all(token_string.as_bytes())
            .expect("Expected to write to file.");

        if curr_token.token_kind == TokenKind::EOF || curr_token.token_kind == TokenKind::ERROR
        {
            break;
        }
    }

    // Then, assert that the output file is the same as the solution file.
    let output = std::fs::read_to_string(output_filepath).expect("Unable to read output file");
    let solution = std::fs::read_to_string(solution_filepath).expect("Unable to read solution file");

    assert_eq!(output, solution, "Output file differs from solution file");
}

// Recogniser tests are next.
