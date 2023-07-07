use std::collections::HashSet;

use log::{debug, error, info, warn};

use crate::globals::TAB_SIZE;
use crate::scanner::scanner_handlers::handle_tokens;
use crate::token::{Token, TokenKind};
use crate::utils::SourcePosition;

pub mod scanner_handlers;

/*
   Scanner
   Scanner's purpose is two folder:

   1. Accept a string of the file contents.
   2. Return the next token in the file contents.
   3. Advance the character number per number of characters in the token.

   Parser will then take the Token and operate upon it.
*/
#[derive(Clone, Debug)]
pub struct ScannerProductType {
    file_contents: Vec<char>,
    curr_source_pos: SourcePosition,
    curr_char_index: usize,
    curr_token_spelling: String,
    final_token_kind: TokenKind,
}

impl std::fmt::Display for ScannerProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ScannerProductType {{ curr_source_pos: {:?}, curr_char_index: {:?}, curr_token_spelling: {:?}, final_token_kind: {:?} }}",
            self.curr_source_pos, self.curr_char_index, self.curr_token_spelling, self.final_token_kind
        )
    }
}

#[derive(Clone, Debug)]
pub struct Scanner {
    global_source_position: SourcePosition,
    global_character_index: usize,
    file_contents: Vec<char>,
}

impl Scanner {
    pub fn new(s: String) -> Self {
        Self {
            global_source_position: SourcePosition::new(1, 1, 1, 1),
            global_character_index: 0,
            file_contents: s.chars().collect(),
        }
    }
    pub fn get_next_token(&mut self) -> Token {
        let mut product_type = ScannerProductType {
            file_contents: self.file_contents.clone(),
            curr_source_pos: self.global_source_position.clone(),
            curr_char_index: self.global_character_index,
            curr_token_spelling: "".to_string(),
            final_token_kind: TokenKind::ERROR,
        };
        let final_product_type = get_token(&mut product_type);

        self.global_source_position = final_product_type.curr_source_pos.clone();
        self.global_source_position.char_start = self.global_source_position.char_end + 1;
        self.global_source_position.char_end = self.global_source_position.char_end + 1;
        self.global_character_index = final_product_type.curr_char_index.clone();

        Token {
            token_kind: final_product_type.final_token_kind,
            spelling: final_product_type.curr_token_spelling,
            token_position: final_product_type.curr_source_pos,
        }
    }
}

fn get_token(adt: &mut ScannerProductType) -> ScannerProductType {
    skip_spaces_comments_newlines(adt);

    let result = handle_tokens(adt).unwrap();
    let end_diff: i32 = result.curr_token_spelling.len() as i32 - 1;

    ScannerProductType {
        curr_source_pos: SourcePosition::new(
            adt.curr_source_pos.line_start,
            adt.curr_source_pos.line_finish,
            adt.curr_source_pos.char_start,
            adt.curr_source_pos.char_end + end_diff,
        ),
        ..result.clone()
    }
}

pub fn accept_next_character(adt: &mut ScannerProductType) {
    adt.curr_token_spelling
        .push(adt.file_contents[adt.curr_char_index]);

    increment_character_position(adt);
}

pub fn skip_next_character(adt: &mut ScannerProductType) {
    let current_char = adt.file_contents[adt.curr_char_index];
    adt.curr_char_index += 1;
    if current_char == '\n' {
        adt.curr_source_pos.line_start += 1;
        adt.curr_source_pos.line_finish += 1;
        adt.curr_source_pos.char_start = 1;
        adt.curr_source_pos.char_end = 1;
    } else {
        adt.curr_source_pos.char_start += 1;
        adt.curr_source_pos.char_end += 1;
    }
}

fn increment_character_position(adt: &mut ScannerProductType) {
    let current_char = adt.file_contents[adt.curr_char_index];
    adt.curr_char_index += 1;
    if current_char == '\n' {
        adt.curr_source_pos.line_start += 1;
        adt.curr_source_pos.line_finish += 1;
        adt.curr_source_pos.char_start = 1;
        adt.curr_source_pos.char_end = 1;
    } else {
        // adt.curr_source_pos.char_start += 1;
        // adt.curr_source_pos.char_end += 1;
    }
}

fn get_current_char(adt: &mut ScannerProductType) -> char {
    // $ is the end of file by convention.
    adt.file_contents
        .get(adt.curr_char_index)
        .cloned()
        .unwrap_or('$')
}

fn get_next_char(adt: &mut ScannerProductType) -> char {
    adt.file_contents
        .get(adt.curr_char_index + 1)
        .cloned()
        .unwrap_or('`')
}

fn skip_spaces_comments_newlines(adt: &mut ScannerProductType) {
    let current_char = get_current_char(adt);
    let next_char = get_next_char(adt);

    // While current char is a space, tab, newline.
    match (current_char, next_char) {
        ('/', '/') => {
            skip_next_character(adt);
            handle_single_line_comment(adt)
        }
        ('/', '*') => handle_multiline_comment(adt),
        (' ', _) => handle_remove_spaces(adt),
        ('\t', _) => handle_tab(adt),
        ('\n', _) => handle_newline(adt),
        _ => (),
    }
}

fn handle_single_line_comment(adt: &mut ScannerProductType) {
    skip_next_character(adt);
    let curr_char = get_current_char(adt);

    if curr_char != '\n' && curr_char != '$' {
        handle_single_line_comment(adt);
    } else {
        skip_next_character(adt);
        skip_spaces_comments_newlines(adt);
    }
}

fn handle_multiline_comment(adt: &mut ScannerProductType) {
    let curr_char = get_current_char(adt);
    let next_char = get_next_char(adt);

    match (curr_char, next_char) {
        (_, '$') => panic!("Multiline comment not closed"),
        ('*', '/') => {
            skip_next_character(adt);
            skip_next_character(adt);
            skip_spaces_comments_newlines(adt);
        }
        (_, _) => {
            skip_next_character(adt);
            handle_multiline_comment(adt);
        }
    }
}

fn handle_remove_spaces(adt: &mut ScannerProductType) {
    skip_next_character(adt);
    skip_spaces_comments_newlines(adt);
}

fn handle_tab(adt: &mut ScannerProductType) {
    adt.curr_source_pos.char_start += TAB_SIZE - (adt.curr_source_pos.char_start % TAB_SIZE);
    adt.curr_source_pos.char_end += TAB_SIZE - (adt.curr_source_pos.char_end % TAB_SIZE);
    skip_next_character(adt);
    skip_spaces_comments_newlines(adt);
}

fn handle_newline(adt: &mut ScannerProductType) {
    if adt.file_contents[adt.curr_char_index] == '\n' {
        skip_next_character(adt);
    }
    skip_spaces_comments_newlines(adt);
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::scanner::{accept_next_character, ScannerProductType, skip_next_character};
    use crate::token::TokenKind;
    use crate::utils::SourcePosition;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
