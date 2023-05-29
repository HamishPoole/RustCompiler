use std::str::FromStr;

use regex::Regex;

use crate::token::TokenKind;

pub fn print_string(s: &String) {
    println!("{}", s);
    for line in s.lines() {
        println!("{}", line);
    }

    println!("{}", s.escape_default());
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SourcePosition {
    pub line_start: i32,
    pub line_finish: i32,
    pub char_start: i32,
    pub char_end: i32,
}

impl SourcePosition {
    pub fn new(line_start: i32, line_finish: i32, char_start: i32, char_end: i32) -> Self {
        Self {
            line_start,
            line_finish,
            char_start,
            char_end,
        }
    }
}

pub fn print_stuff() {
    // assuming that each line of the message is stored in a vector
    // Use non IDE nvim to format.
    let messages = vec![
        r#"Kind = 7 [int], spelling = "int", position = 1(1)..1(3)"#,
        r#"Kind = 33 [<id>], spelling = "i", position = 1(5)..1(5)"#,
        r#"Kind = 31 [;], spelling = ";", position = 1(6)..1(6)"#,
        r#"Kind = 7 [int], spelling = "int", position = 2(1)..2(3)"#,
        r#"Kind = 33 [<id>], spelling = "j", position = 2(5)..2(5)"#,
        r#"Kind = 31 [;], spelling = ";", position = 2(6)..2(6)"#,
        r#"Kind = 7 [int], spelling = "int", position = 4(1)..4(3)"#,
        r#"Kind = 33 [<id>], spelling = "gcd", position = 4(5)..4(7)"#,
        r#"Kind = 27 [(], spelling = "(", position = 4(8)..4(8)"#,
        r#"Kind = 7 [int], spelling = "int", position = 4(9)..4(11)"#,
        r#"Kind = 33 [<id>], spelling = "a", position = 4(13)..4(13)"#,
        r#"Kind = 32 [,], spelling = ",", position = 4(14)..4(14)"#,
        r#"Kind = 7 [int], spelling = "int", position = 4(16)..4(18)"#,
        r#"Kind = 33 [<id>], spelling = "b", position = 4(20)..4(20)"#,
        r#"Kind = 28 [)], spelling = ")", position = 4(21)..4(21)"#,
        r#"Kind = 25 [{], spelling = "{", position = 4(23)..4(23)"#,
        r#"Kind = 6 [if], spelling = "if", position = 5(3)..5(4)"#,
        r#"Kind = 27 [(], spelling = "(", position = 5(6)..5(6)"#,
        r#"Kind = 33 [<id>], spelling = "b", position = 5(7)..5(7)"#,
        r#"Kind = 18 [==], spelling = "==", position = 5(9)..5(10)"#,
        r#"Kind = 34 [<int-literal>], spelling = "0", position = 5(12)..5(12)"#,
        r#"Kind = 28 [)], spelling = ")", position = 5(13)..5(13)"#,
        r#"Kind = 8 [return], spelling = "return", position = 6(5)..6(10)"#,
        r#"Kind = 33 [<id>], spelling = "a", position = 6(12)..6(12)"#,
        r#"Kind = 31 [;], spelling = ";", position = 6(13)..6(13)"#,
        r#"Kind = 3 [else], spelling = "else", position = 7(3)..7(6)"#,
        r#"Kind = 8 [return], spelling = "return", position = 8(3)..8(8)"#,
        r#"Kind = 33 [<id>], spelling = "gcd", position = 8(10)..8(12)"#,
        r#"Kind = 27 [(], spelling = "(", position = 8(13)..8(13)"#,
        r#"Kind = 33 [<id>], spelling = "b", position = 8(14)..8(14)"#,
        r#"Kind = 32 [,], spelling = ",", position = 8(15)..8(15)"#,
        r#"Kind = 33 [<id>], spelling = "a", position = 8(17)..8(17)"#,
        r#"Kind = 12 [-], spelling = "-", position = 8(19)..8(19)"#,
        r#"Kind = 27 [(], spelling = "(", position = 8(21)..8(21)"#,
        r#"Kind = 33 [<id>], spelling = "a", position = 8(22)..8(22)"#,
        r#"Kind = 14 [/], spelling = "/", position = 8(23)..8(23)"#,
        r#"Kind = 33 [<id>], spelling = "b", position = 8(24)..8(24)"#,
        r#"Kind = 28 [)], spelling = ")", position = 8(25)..8(25)"#,
        r#"Kind = 13 [*], spelling = "*", position = 8(27)..8(27)"#,
        r#"Kind = 33 [<id>], spelling = "b", position = 8(28)..8(28)"#,
        r#"Kind = 28 [)], spelling = ")", position = 8(29)..8(29)"#,
        r#"Kind = 31 [;], spelling = ";", position = 8(30)..8(30)"#,
        r#"Kind = 26 [}], spelling = "}", position = 9(1)..9(1)"#,
        r#"Kind = 9 [void], spelling = "void", position = 10(1)..10(4)"#,
        r#"Kind = 33 [<id>], spelling = "main", position = 10(6)..10(9)"#,
        r#"Kind = 27 [(], spelling = "(", position = 10(10)..10(10)"#,
        r#"Kind = 28 [)], spelling = ")", position = 10(11)..10(11)"#,
        r#"Kind = 25 [{], spelling = "{", position = 10(13)..10(13)"#,
        r#"Kind = 33 [<id>], spelling = "i", position = 11(3)..11(3)"#,
        r#"Kind = 17 [=], spelling = "=", position = 11(5)..11(5)"#,
        r#"Kind = 33 [<id>], spelling = "getInt", position = 11(7)..11(12)"#,
        r#"Kind = 27 [(], spelling = "(", position = 11(13)..11(13)"#,
        r#"Kind = 28 [)], spelling = ")", position = 11(14)..11(14)"#,
        r#"Kind = 31 [;], spelling = ";", position = 11(15)..11(15)"#,
        r#"Kind = 33 [<id>], spelling = "j", position = 12(3)..12(3)"#,
        r#"Kind = 17 [=], spelling = "=", position = 12(5)..12(5)"#,
        r#"Kind = 33 [<id>], spelling = "getInt", position = 12(7)..12(12)"#,
        r#"Kind = 27 [(], spelling = "(", position = 12(13)..12(13)"#,
        r#"Kind = 28 [)], spelling = ")", position = 12(14)..12(14)"#,
        r#"Kind = 31 [;], spelling = ";", position = 12(15)..12(15)"#,
        r#"Kind = 33 [<id>], spelling = "putIntLn", position = 13(3)..13(10)"#,
        r#"Kind = 27 [(], spelling = "(", position = 13(11)..13(11)"#,
        r#"Kind = 33 [<id>], spelling = "gcd", position = 13(12)..13(14)"#,
        r#"Kind = 27 [(], spelling = "(", position = 13(15)..13(15)"#,
        r#"Kind = 33 [<id>], spelling = "i", position = 13(16)..13(16)"#,
        r#"Kind = 32 [,], spelling = ",", position = 13(17)..13(17)"#,
        r#"Kind = 33 [<id>], spelling = "j", position = 13(19)..13(19)"#,
        r#"Kind = 28 [)], spelling = ")", position = 13(20)..13(20)"#,
        r#"Kind = 28 [)], spelling = ")", position = 13(21)..13(21)"#,
        r#"Kind = 31 [;], spelling = ";", position = 13(22)..13(22)"#,
        r#"Kind = 26 [}], spelling = "}", position = 14(1)..14(1)"#,
        r#"Kind = 39 [$], spelling = "$", position = 15(1)..15(1)"#,
    ];

    for message in messages {
        // split each line by commas
        let parts: Vec<&str> = message.split(", ").collect();

        let kind_parts: Vec<&str> = parts[0].split('[').collect();
        let token_kind = kind_parts[1].trim_end_matches(']');
        let token_spelling = parts[1].split('\"').nth(1).unwrap_or("N/A");

        let position_parts: Vec<&str> = parts[2].split("..").collect();
        let line_start_parts: Vec<&str> = position_parts[0]
            .trim_start_matches("position = ")
            .split('(')
            .collect();
        let line_finish_parts: Vec<&str> = position_parts[1].split('(').collect();

        let line_start = line_start_parts.get(0).unwrap_or(&"N/A");
        let char_start = line_start_parts.get(1).map(|s| s.trim_end_matches(')'));
        let char_start = char_start.unwrap_or(line_start_parts.get(1).unwrap());
        let line_finish = line_finish_parts.get(0).unwrap_or(&"N/A");
        let char_end = line_finish_parts
            .get(1)
            .map(|s| s.trim_end_matches(')'))
            .unwrap_or("N/A"); // print the reformatted message

        println!(
            "ScannerToken {{ token_kind: {:?}, token_spelling: \"{}\", source_position: SourcePosition {{ line_start: {}, line_finish: {}, char_start: {}, char_end: {} }} }}",
            TokenKind::from_str(token_spelling).unwrap(), token_spelling, line_start, line_finish, char_start, char_end
        );
    }
}
