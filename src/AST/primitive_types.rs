use std::fmt;
use std::sync::Arc;

use crate::ast::Ast;
use crate::ast::expression::ExprType;
use crate::utils::SourcePosition;

#[derive(Clone, Debug)]
pub enum PrimitiveType {
    BooleanType(BooleanType),
    ErrorType(ErrorType),
    FloatType(FloatType),
    IntType(IntType),
    StringType(StringType),
    VoidType(VoidType),
}

// #[derive(Clone, Debug)]
// pub enum PrimitiveNonRecursive {
//     BooleanType(BooleanType),
//     ErrorType(ErrorType),
//     FloatType(FloatType),
//     IntType(IntType),
//     StringType(StringType),
//     VoidType(VoidType),
// }

#[derive(Debug)]
pub struct ArrayType {
    source_position: SourcePosition,
    array_type: Arc<Box<PrimitiveType>>,
    expression: ExprType,
}

impl Ast for ArrayType {
    fn visit(&self) {
        println!("Visiting ArrayType node.");
        // Implement visitArrayType function...
    }
}

impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayType")
    }
}

impl ArrayType {
    pub fn new(
        source_position: SourcePosition,
        array_type: Arc<Box<PrimitiveType>>,
        expression: ExprType,
    ) -> Self {
        Self {
            source_position,
            array_type,
            expression,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BooleanType {
    source_position: SourcePosition,
}

impl Ast for BooleanType {
    fn visit(&self) {
        println!("Visiting BooleanType node.");
        // Implement visitBooleanType function...
    }
}

impl fmt::Display for BooleanType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BooleanType")
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
    fn visit(&self) {
        println!("Visiting ErrorType node.");
        // Implement visitErrorType function...
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorType")
    }
}

#[derive(Clone, Debug)]
pub struct FloatType {
    source_position: SourcePosition,
}

impl Ast for FloatType {
    fn visit(&self) {
        println!("Visiting FloatType node.");
        // Implement visitFloatType function...
    }
}

impl fmt::Display for FloatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FloatType")
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
    fn visit(&self) {
        println!("Visiting IntType node.");
        // Implement visitIntType function...
    }
}

impl fmt::Display for IntType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IntType")
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
    fn visit(&self) {
        println!("Visiting StringType node.");
        // Implement visitStringType function...
    }
}

impl fmt::Display for StringType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StringType")
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
    fn visit(&self) {
        println!("Visiting VoidType node.");
        // Implement visitVoidType function...
    }
}

impl fmt::Display for VoidType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "void")
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
