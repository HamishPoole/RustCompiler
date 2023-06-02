use std::fmt;
use std::sync::Arc;

use crate::ast::{Ast, PrintingVisit};
use crate::ast::expression::ExprType;
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Clone, Debug)]
pub enum AstTypes {
    BooleanType(BooleanType),
    FloatType(FloatType),
    IntType(IntType),
    StringType(StringType),
    VoidType(VoidType),
    ErrorType(ErrorType),
}

impl PrintingVisit for AstTypes {
    fn visit_for_printing(&self, depth: i32) {
        match self {
            AstTypes::BooleanType(boolean_type) => boolean_type.visit_for_printing(depth),
            AstTypes::FloatType(float_type) => float_type.visit_for_printing(depth),
            AstTypes::IntType(int_type) => int_type.visit_for_printing(depth),
            AstTypes::StringType(string_type) => string_type.visit_for_printing(depth),
            AstTypes::VoidType(void_type) => void_type.visit_for_printing(depth),
            AstTypes::ErrorType(error_type) => error_type.visit_for_printing(depth),
        }
    }
}

impl AstTypes {
    fn is_primitive_type(&self) -> bool {
        match self {
            AstTypes::BooleanType(_) => true,
            AstTypes::FloatType(_) => true,
            AstTypes::IntType(_) => true,
            AstTypes::VoidType(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BooleanType {
    source_position: SourcePosition,
}

impl Ast for BooleanType {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting BooleanType node.");
        // Implement visitBooleanType function...
    }
}

impl fmt::Display for BooleanType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BooleanType")
    }
}

impl PrintingVisit for BooleanType {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl BooleanType {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Clone, Debug)]
pub struct ErrorType {
    source_position: SourcePosition,
}

impl Ast for ErrorType {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ErrorType node.");
        // Implement visitErrorType function...
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorType")
    }
}

impl PrintingVisit for ErrorType {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

#[derive(Clone, Debug)]
pub struct FloatType {
    source_position: SourcePosition,
}

impl Ast for FloatType {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting FloatType node.");
        // Implement visitFloatType function...
    }
}

impl fmt::Display for FloatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FloatType")
    }
}

impl PrintingVisit for FloatType {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl FloatType {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntType {
    source_position: SourcePosition,
}

impl Ast for IntType {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntType node.");
        // Implement visitIntType function...
    }
}

impl fmt::Display for IntType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IntType")
    }
}

impl PrintingVisit for IntType {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl IntType {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Clone, Debug)]
pub struct StringType {
    source_position: SourcePosition,
}

impl Ast for StringType {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting StringType node.");
        // Implement visitStringType function...
    }
}

impl fmt::Display for StringType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StringType")
    }
}

impl PrintingVisit for StringType {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl StringType {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Clone, Debug)]
pub struct VoidType {
    source_position: SourcePosition,
}

impl Ast for VoidType {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting VoidType node.");
        // Implement visitVoidType function...
    }
}

impl fmt::Display for VoidType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "void")
    }
}

impl PrintingVisit for VoidType {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl VoidType {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}
// Many Rust programmers use compile time macros to generate large blocks of code like this.
// However, I prefer ChatGPT.
// Comment driven development is the future of programming.
// Defining the system you want to create in words parsable by ChatGPT.
// English language proficiency + ChatGPT = 10x productivity.
// Massive boost to researchers.
