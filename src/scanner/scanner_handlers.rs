use std::collections::HashSet;
use std::str::FromStr;

use log::error;

use crate::scanner::{
    accept_next_character, get_current_char, get_next_char, skip_next_character, ScannerProductType,
};
use crate::token::TokenKind;

pub fn handle_tokens(adt: &mut ScannerProductType) -> Result<ScannerProductType, String> {
    let a = 2;

    if let Ok(new_adt) = handle_special(adt) {
        Ok(new_adt)
    } else if let Ok(new_adt) = handle_separators(adt) {
        Ok(new_adt)
    } else if let Ok(new_adt) = handle_operators(adt) {
        Ok(new_adt)
    } else if let Ok(new_adt) = handle_literals(adt) {
        Ok(new_adt)
    } else if let Ok(new_adt) = handle_identifiers(adt) {
        Ok(new_adt)
    } else {
        Err(String::from(format!(
            "No token matched in handle_tokens.  adt: {adt}",
        )))
    }
}

pub fn handle_special(adt: &mut ScannerProductType) -> Result<ScannerProductType, String> {
    if adt.curr_char_index >= adt.file_contents.len() {
        return Ok(ScannerProductType {
            final_token_kind: TokenKind::EOF,
            curr_token_spelling: String::from("$"),
            ..adt.clone()
        });
    }
    Err(String::from("Unhandled special token"))
}

pub fn handle_separators(adt: &mut ScannerProductType) -> Result<ScannerProductType, String> {
    let current_char = adt.file_contents[adt.curr_char_index];

    log::debug!("Reached handle_separators");
    log::debug!("Current char: {}", current_char);
    // TODO: Brush up debug logs.
    match current_char {
        '(' => {
            log::debug!("Reached ( match");
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::LPAREN,
                ..adt.clone()
            })
        }
        ')' => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::RPAREN,
                ..adt.clone()
            })
        }
        '{' => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::LBRACE,
                ..adt.clone()
            })
        }
        '}' => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::RBRACE,
                ..adt.clone()
            })
        }
        '[' => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::LBRACKET,
                ..adt.clone()
            })
        }
        ']' => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::RBRACKET,
                ..adt.clone()
            })
        }
        ';' => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::SEMICOLON,
                ..adt.clone()
            })
        }
        ',' => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::COMMA,
                ..adt.clone()
            })
        }
        _ => Err(String::from("No match found in separators.")),
    }
}

fn handle_operators(adt: &mut ScannerProductType) -> Result<ScannerProductType, String> {
    let current_char = get_current_char(adt);
    let next_char = get_next_char(adt);

    log::debug!("Reached handle_operators");
    log::debug!("Current char: {}", current_char);

    match (current_char, next_char) {
        ('+', next_char) => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::PLUS,
                ..adt.clone()
            })
        }
        ('-', _) => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::MINUS,
                ..adt.clone()
            })
        }
        ('*', _) => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::MULT,
                ..adt.clone()
            })
        }
        ('/', _) => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::DIV,
                ..adt.clone()
            })
        }
        ('!', '=') => {
            accept_next_character(adt);
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::NOTEQ,
                ..adt.clone()
            })
        }
        ('!', _) => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::NOT,
                ..adt.clone()
            })
        }
        ('=', '=') => {
            accept_next_character(adt);
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::EQEQ,
                ..adt.clone()
            })
        }
        ('=', _) => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::EQ,
                ..adt.clone()
            })
        }
        ('<', '=') => {
            accept_next_character(adt);
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::LTEQ,
                ..adt.clone()
            })
        }
        ('<', _) => {
            accept_next_character(adt);
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::LT,
                ..adt.clone()
            })
        }
        ('>', '=') => {
            accept_next_character(adt);
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::GTEQ,
                ..adt.clone()
            })
        }
        ('>', _) => {
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::GT,
                ..adt.clone()
            })
        }
        ('&', '&') => {
            accept_next_character(adt);
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::ANDAND,
                ..adt.clone()
            })
        }
        ('|', '|') => {
            accept_next_character(adt);
            accept_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::OROR,
                ..adt.clone()
            })
        }
        _ => Err(String::from("No match found in operators.")),
    }
}

fn handle_literals(adt: &mut ScannerProductType) -> Result<ScannerProductType, String> {
    let curr_char = get_current_char(adt);
    let next_char = get_next_char(adt);

    match (curr_char, next_char) {
        (c, _) if c.is_numeric() || (c == '.' && next_char.is_numeric()) => handle_numbers(adt),
        ('"', _) => {
            skip_next_character(adt); // Absorbs the first quote.
            handle_strings(adt)
        }
        _ => Err(String::from("No match found in literals.")),
    }
}

fn handle_identifiers(adt: &mut ScannerProductType) -> Result<ScannerProductType, String> {
    let curr_char = get_current_char(adt);

    match curr_char {
        c if c.is_alphabetic() || c == '_' => {
            accept_next_character(adt);
            handle_parse_ids(adt)
        }
        _ => Err(String::from("No match found in identifiers.")),
    }
}

fn handle_parse_ids(adt: &mut ScannerProductType) -> Result<ScannerProductType, String> {
    let curr_char = get_current_char(adt);
    let next_char = get_next_char(adt);

    match (curr_char, next_char) {
        (c, _) if c.is_alphabetic() || c == '_' => handle_identifiers(adt),
        _ => {
            error!(
                "TokenKind from str {:?}",
                TokenKind::from_str(&adt.curr_token_spelling)
            );

            error!("Current spelling: {:?}", adt.curr_token_spelling);

            if adt.curr_token_spelling == "true" || adt.curr_token_spelling == "false" {
                Ok(ScannerProductType {
                    final_token_kind: TokenKind::BOOLEANLITERAL,
                    ..adt.clone()
                })
            } else if TokenKind::from_str(&adt.curr_token_spelling).is_ok() {
                // Handles if the string has spelt a keyword.
                Ok(ScannerProductType {
                    final_token_kind: TokenKind::from_str(&adt.curr_token_spelling).unwrap(),
                    ..adt.clone()
                })
            } else {
                Ok(ScannerProductType {
                    final_token_kind: TokenKind::ID,
                    ..adt.clone()
                })
            }
        }
    }
}

fn handle_numbers(adt: &mut ScannerProductType) -> Result<ScannerProductType, String> {
    let curr_char = get_current_char(adt);
    let next_char = get_next_char(adt);

    match curr_char {
        curr_char if curr_char.is_numeric() => {
            accept_next_character(adt);
            handle_numbers(adt)
        }
        '.' | 'e' | 'E' => {
            accept_next_character(adt);
            handle_floats(adt)
        }
        _ => Ok(ScannerProductType {
            final_token_kind: TokenKind::INTLITERAL,
            ..adt.clone()
        }),
    }
}

fn handle_floats(adt: &mut ScannerProductType) -> Result<ScannerProductType, String> {
    if get_current_char(adt) == '.' {
        accept_next_character(adt);
        while get_current_char(adt).is_numeric() {
            accept_next_character(adt);
        }
    }

    let curr_char = get_current_char(adt);

    if curr_char == 'e' || curr_char == 'E' {
        accept_next_character(adt);
        let curr_char = get_current_char(adt);
        if curr_char == '+' || curr_char == '-' {
            accept_next_character(adt);
        }
        while get_current_char(adt).is_numeric() {
            accept_next_character(adt);
        }
    }

    Ok(ScannerProductType {
        final_token_kind: TokenKind::FLOAT,
        ..adt.clone()
    })
}

fn handle_strings(adt: &mut ScannerProductType) -> Result<ScannerProductType, String> {
    // Skip the initial quote

    let curr_char = get_current_char(adt);
    let next_char = get_next_char(adt);

    match (curr_char, next_char) {
        ('"', _) => {
            skip_next_character(adt);
            Ok(ScannerProductType {
                final_token_kind: TokenKind::STRINGLITERAL,
                ..adt.clone()
            })
        }
        ('\\', _) => {
            // Generate a hash set of valid escape characters.
            let escapes: HashSet<char> = ['n', 't', 'r', 'f', 'b', '\\', '"']
                .iter()
                .cloned()
                .collect();
            if escapes.contains(&next_char) {
                accept_next_character(adt);
                accept_next_character(adt);
                handle_strings(adt)
            } else {
                Err(String::from("Invalid escape character: {next_char}"))
            }
        }
        ('\n', _) | ('\r', _) => Err(String::from(
            "Error: Unterminated string literal.  adt: {adt}",
        )),
        (_, _) => {
            accept_next_character(adt);
            handle_strings(adt)
        }
    }
}
