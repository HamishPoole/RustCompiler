use std::str::FromStr;

use regex::Regex;

use crate::globals::TAB_SIZE;
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

pub fn convert_test_format() {
    // assuming that each line of the message is stored in a vector
    // Use non IDE nvim to format.
    let gcd_solution = vec![
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


    let fib_solution = vec![
        r#"Kind = 9 [void], spelling = "void", position = 2(1)..2(4)"#,
        r#"Kind = 33 [<id>], spelling = "main", position = 3(6)..3(9)"#,
        r#"Kind = 27 [(], spelling = "(", position = 3(10)..3(10)"#,
        r#"Kind = 28 [)], spelling = ")", position = 3(11)..3(11)"#,
        r#"Kind = 25 [{], spelling = "{", position = 3(13)..3(13)"#,
        r#"Kind = 7 [int], spelling = "int", position = 4(5)..4(7)"#,
        r#"Kind = 33 [<id>], spelling = "n", position = 4(9)..4(9)"#,
        r#"Kind = 31 [;], spelling = ";", position = 4(10)..4(10)"#,
        r#"Kind = 7 [int], spelling = "int", position = 5(5)..5(7)"#,
        r#"Kind = 33 [<id>], spelling = "i", position = 5(9)..5(9)"#,
        r#"Kind = 31 [;], spelling = ";", position = 5(10)..5(10)"#,
        r#"Kind = 7 [int], spelling = "int", position = 6(5)..6(7)"#,
        r#"Kind = 33 [<id>], spelling = "current", position = 6(9)..6(15)"#,
        r#"Kind = 31 [;], spelling = ";", position = 6(16)..6(16)"#,
        r#"Kind = 7 [int], spelling = "int", position = 7(5)..7(7)"#,
        r#"Kind = 33 [<id>], spelling = "next", position = 7(9)..7(12)"#,
        r#"Kind = 31 [;], spelling = ";", position = 7(13)..7(13)"#,
        r#"Kind = 7 [int], spelling = "int", position = 8(5)..8(7)"#,
        r#"Kind = 33 [<id>], spelling = "twoaway", position = 8(9)..8(15)"#,
        r#"Kind = 31 [;], spelling = ";", position = 8(16)..8(16)"#,
        r#"Kind = 33 [<id>], spelling = "putString", position = 10(5)..10(13)"#,
        r#"Kind = 27 [(], spelling = "(", position = 10(14)..10(14)"#,
        r#"Kind = 37 [<string-literal>], spelling = "How many Fibonacci numbers do you want to compute? ", position = 10(15)..10(67)"#,
        r#"Kind = 28 [)], spelling = ")", position = 10(68)..10(68)"#,
        r#"Kind = 31 [;], spelling = ";", position = 10(69)..10(69)"#,
        r#"Kind = 33 [<id>], spelling = "n", position = 11(5)..11(5)"#,
        r#"Kind = 17 [=], spelling = "=", position = 11(7)..11(7)"#,
        r#"Kind = 33 [<id>], spelling = "getInt", position = 11(9)..11(14)"#,
        r#"Kind = 27 [(], spelling = "(", position = 11(15)..11(15)"#,
        r#"Kind = 28 [)], spelling = ")", position = 11(16)..11(16)"#,
        r#"Kind = 31 [;], spelling = ";", position = 11(17)..11(17)"#,
        r#"Kind = 6 [if], spelling = "if", position = 12(5)..12(6)"#,
        r#"Kind = 27 [(], spelling = "(", position = 12(8)..12(8)"#,
        r#"Kind = 33 [<id>], spelling = "n", position = 12(9)..12(9)"#,
        r#"Kind = 20 [<=], spelling = "<=", position = 12(10)..12(11)"#,
        r#"Kind = 34 [<int-literal>], spelling = "0", position = 12(12)..12(12)"#,
        r#"Kind = 28 [)], spelling = ")", position = 12(13)..12(13)"#,
        r#"Kind = 33 [<id>], spelling = "putString", position = 13(8)..13(16)"#,
        r#"Kind = 27 [(], spelling = "(", position = 13(17)..13(17)"#,
        r#"Kind = 37 [<string-literal>], spelling = "The number should be positive.", position = 13(18)..13(51)"#,
        r#"Kind = 28 [)], spelling = ")", position = 13(52)..13(52)"#,
        r#"Kind = 31 [;], spelling = ";", position = 13(53)..13(53)"#,
        r#"Kind = 3 [else], spelling = "else", position = 14(5)..14(8)"#,
        r#"Kind = 25 [{], spelling = "{", position = 14(10)..14(10)"#,
        r#"Kind = 33 [<id>], spelling = "putString", position = 15(7)..15(15)"#,
        r#"Kind = 27 [(], spelling = "(", position = 15(16)..15(16)"#,
        r#"Kind = 28 [)], spelling = ")", position = 15(70)..15(70)"#,
        r#"Kind = 31 [;], spelling = ";", position = 15(71)..15(71)"#,
        r#"Kind = 33 [<id>], spelling = "next", position = 16(7)..16(10)"#,
        r#"Kind = 17 [=], spelling = "=", position = 16(12)..16(12)"#,
        r#"Kind = 33 [<id>], spelling = "current", position = 16(14)..16(20)"#,
        r#"Kind = 17 [=], spelling = "=", position = 16(22)..16(22)"#,
        r#"Kind = 34 [<int-literal>], spelling = "1", position = 16(24)..16(24)"#,
        r#"Kind = 31 [;], spelling = ";", position = 16(25)..16(25)"#,
        r#"Kind = 5 [for], spelling = "for", position = 17(7)..17(9)"#,
        r#"Kind = 27 [(], spelling = "(", position = 17(11)..17(11)"#,
        r#"Kind = 33 [<id>], spelling = "i", position = 17(12)..17(12)"#,
        r#"Kind = 17 [=], spelling = "=", position = 17(13)..17(13)"#,
        r#"Kind = 34 [<int-literal>], spelling = "1", position = 17(14)..17(14)"#,
        r#"Kind = 31 [;], spelling = ";", position = 17(15)..17(15)"#,
        r#"Kind = 33 [<id>], spelling = "i", position = 17(17)..17(17)"#,
        r#"Kind = 20 [<=], spelling = "<=", position = 17(18)..17(19)"#,
        r#"Kind = 33 [<id>], spelling = "n", position = 17(20)..17(20)"#,
        r#"Kind = 31 [;], spelling = ";", position = 17(21)..17(21)"#,
        r#"Kind = 33 [<id>], spelling = "i", position = 17(23)..17(23)"#,
        r#"Kind = 17 [=], spelling = "=", position = 17(24)..17(24)"#,
        r#"Kind = 33 [<id>], spelling = "i", position = 17(25)..17(25)"#,
        r#"Kind = 11 [+], spelling = "+", position = 17(26)..17(26)"#,
        r#"Kind = 34 [<int-literal>], spelling = "1", position = 17(27)..17(27)"#,
        r#"Kind = 28 [)], spelling = ")", position = 17(28)..17(28)"#,
        r#"Kind = 25 [{], spelling = "{", position = 17(30)..17(30)"#,
        r#"Kind = 33 [<id>], spelling = "putString", position = 18(9)..18(17)"#,
        r#"Kind = 27 [(], spelling = "(", position = 18(18)..18(18)"#,
        r#"Kind = 37 [<string-literal>], spelling = "	", position = 18(19)..18(22)"#,
        r#"Kind = 28 [)], spelling = ")", position = 18(23)..18(23)"#,
        r#"Kind = 31 [;], spelling = ";", position = 18(24)..18(24)"#,
        r#"Kind = 33 [<id>], spelling = "putInt", position = 19(9)..19(14)"#,
        r#"Kind = 27 [(], spelling = "(", position = 19(15)..19(15)"#,
        r#"Kind = 33 [<id>], spelling = "i", position = 19(16)..19(16)"#,
        r#"Kind = 28 [)], spelling = ")", position = 19(17)..19(17)"#,
        r#"Kind = 31 [;], spelling = ";", position = 19(18)..19(18)"#,
        r#"Kind = 33 [<id>], spelling = "putString", position = 20(9)..20(17)"#,
        r#"Kind = 27 [(], spelling = "(", position = 20(18)..20(18)"#,
        r#"Kind = 37 [<string-literal>], spelling = "	", position = 20(19)..20(22)"#,
        r#"Kind = 28 [)], spelling = ")", position = 20(23)..20(23)"#,
        r#"Kind = 31 [;], spelling = ";", position = 20(24)..20(24)"#,
        r#"Kind = 33 [<id>], spelling = "putIntLn", position = 21(9)..21(16)"#,
        r#"Kind = 27 [(], spelling = "(", position = 21(17)..21(17)"#,
        r#"Kind = 33 [<id>], spelling = "current", position = 21(18)..21(24)"#,
        r#"Kind = 28 [)], spelling = ")", position = 21(25)..21(25)"#,
        r#"Kind = 31 [;], spelling = ";", position = 21(26)..21(26)"#,
        r#"Kind = 33 [<id>], spelling = "twoaway", position = 22(9)..22(15)"#,
        r#"Kind = 17 [=], spelling = "=", position = 22(17)..22(17)"#,
        r#"Kind = 33 [<id>], spelling = "current", position = 22(19)..22(25)"#,
        r#"Kind = 11 [+], spelling = "+", position = 22(26)..22(26)"#,
        r#"Kind = 33 [<id>], spelling = "next", position = 22(27)..22(30)"#,
        r#"Kind = 31 [;], spelling = ";", position = 22(31)..22(31)"#,
        r#"Kind = 33 [<id>], spelling = "current", position = 23(9)..23(15)"#,
        r#"Kind = 17 [=], spelling = "=", position = 23(17)..23(17)"#,
        r#"Kind = 33 [<id>], spelling = "next", position = 23(19)..23(22)"#,
        r#"Kind = 31 [;], spelling = ";", position = 23(23)..23(23)"#,
        r#"Kind = 33 [<id>], spelling = "next", position = 24(9)..24(12)"#,
        r#"Kind = 17 [=], spelling = "=", position = 24(17)..24(17)"#,
        r#"Kind = 33 [<id>], spelling = "twoaway", position = 24(19)..24(25)"#,
        r#"Kind = 31 [;], spelling = ";", position = 24(26)..24(26)"#,
        r#"Kind = 26 [}], spelling = "}", position = 25(7)..25(7)"#,
        r#"Kind = 26 [}], spelling = "}", position = 26(5)..26(5)"#,
        r#"Kind = 26 [}], spelling = "}", position = 27(1)..27(1)"#,
        r#"Kind = 39 [$], spelling = "$", position = 47(1)..47(1)"#,
    ];
    // r#"Kind = 28 [)], spelling = ")", position = 15(70)..15(70)"#,

    for message in fib_solution {
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

        // println!("{:?} position parts.", position_parts);
        let line_finish_parts: Vec<&str> = position_parts[1].splitn(2, |c| c == '(' && !position_parts[1].starts_with("[(")).collect();

        let line_start = line_start_parts.get(0).unwrap_or(&"N/A");
        let char_start = line_start_parts.get(1).map(|s| s.trim_end_matches(')'));
        let char_start = char_start.unwrap_or(line_start_parts.get(1).unwrap());
        let line_finish = line_finish_parts.get(0).unwrap_or(&"N/A");
        let char_end = line_finish_parts
            .get(1)
            .map(|s| s.trim_end_matches(')'))
            .unwrap_or("N/A"); // print the reformatted message

        println!(
            "Token {{ token_kind: {:?}, spelling: \"{}\", token_position: SourcePosition {{ line_start: {}, line_finish: {}, char_start: {}, char_end: {} }} }}",
            TokenKind::from_str(token_spelling).unwrap(), token_spelling, line_start, line_finish, char_start, char_end
        );
    }
}

pub fn generate_tabbed_string(s: &str, depth: i32) -> String {
    let node_name = s.rsplit("::").next().unwrap();
    let indent = " ".repeat((depth * TAB_SIZE) as usize);
    indent + node_name
}