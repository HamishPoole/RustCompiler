use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;

use once_cell::sync::Lazy;

use crate::ast::PrintUnparsedAST;
use crate::ast::program::Program;

fn add_escape_to_string(s: &str) -> String {
    let mut t = String::new();
    for c in s.chars() {
        if ESCAPE_CHARS.contains(&c) {
            match c {
                '\x08' => t.push_str("\\b"),
                '\x0C' => t.push_str("\\f"),
                '\n' => t.push_str("\\n"),
                '\r' => t.push_str("\\r"),
                '\t' => t.push_str("\\t"),
                '\'' => t.push_str("\\'"),
                '\"' => t.push_str("\\\""),
                '\\' => t.push_str("\\\\"),
                _ => (),
            }
        } else {
            t.push(c);
        }
    }
    t
}

/*
NB.  Note:

\x08 represents \b or backspace in Rust's string literals.
\x0C represents \f or form feed in Rust's string literals.

See https://www.ascii-code.com/ for ASCII character code 08.
 */
static ESCAPE_CHARS: Lazy<HashSet<char>> = Lazy::new(
    || {
        [
            '\x08', '\x0C', '\n', '\r', '\t', '\'', '\"', '\\'
        ].iter().cloned().collect()
    });

// Escape Strings
static ESCAPE_STRINGS: Lazy<HashSet<&str>> = Lazy::new(
    || {
        [
            "\\b", "\\f", "\\n", "\\r", "\\t", "\\\'", "\\\"", "\\\\"
        ].iter().cloned().collect()
    });
