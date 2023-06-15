use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::ast::{Checking, PrintAST, PrintUnparsedAST};
use crate::globals::TAB_SIZE;
use crate::utils::{generate_indent, generate_tabbed_string, SourcePosition};

#[derive(Clone, Debug, PartialEq)]
pub struct Operator {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Checking for Operator {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintAST for Operator {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl PrintUnparsedAST for Operator {
    fn unparse_to_code(&self, depth: i32) {
        print!("{}", self.spelling)
    }
}

impl Operator {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Terminal {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Checking for Terminal {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintAST for Terminal {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl PrintUnparsedAST for Terminal {
    fn unparse_to_code(&self, depth: i32) {
        print!("{}", self.spelling);
    }
}

impl Terminal {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntLiteral {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Checking for IntLiteral {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for IntLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintAST for IntLiteral {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl PrintUnparsedAST for IntLiteral {
    fn unparse_to_code(&self, depth: i32) {
        print!("{}", self.spelling);
    }
}

impl IntLiteral {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FloatLiteral {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Checking for FloatLiteral {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for FloatLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintAST for FloatLiteral {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl PrintUnparsedAST for FloatLiteral {
    fn unparse_to_code(&self, depth: i32) {
        print!("{}", self.spelling);
    }
}

impl FloatLiteral {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BooleanLiteral {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Checking for BooleanLiteral {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for BooleanLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintAST for BooleanLiteral {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl PrintUnparsedAST for BooleanLiteral {
    fn unparse_to_code(&self, depth: i32) {
        print!("{}", self.spelling);
    }
}

impl BooleanLiteral {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringLiteral {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Checking for StringLiteral {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintAST for StringLiteral {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl PrintUnparsedAST for StringLiteral {
    fn unparse_to_code(&self, depth: i32) {
        print!("{}", self.spelling);
    }
}

impl StringLiteral {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}
