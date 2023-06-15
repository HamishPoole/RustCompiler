use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;

use crate::ast::{Checking, PrintAST, PrintUnparsedAST};
use crate::ast::expression::ExprType;
use crate::ast::primitive_types::AstTypes;
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayType {
    source_position: SourcePosition,
    pub array_type: Arc<Box<AstTypes>>,
    pub expression: ExprType,
}

impl Checking for ArrayType {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ArrayType node.");
        // Implement visitArrayType function...
    }
}

impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayType")
    }
}

impl PrintAST for ArrayType {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.array_type.visit_for_printing(depth + 1);
        self.expression.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for ArrayType {
    fn unparse_to_code(&self, depth: i32) {
        self.array_type.unparse_to_code(depth);
    }
}

impl ArrayType {
    pub fn new(
        source_position: SourcePosition,
        array_type: Arc<Box<AstTypes>>,
        expression: ExprType,
    ) -> Self {
        Self {
            source_position,
            array_type,
            expression,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstTypeVariant {
    Primitive(AstTypes),
    Array(ArrayType),
}

impl PrintAST for AstTypeVariant {
    fn visit_for_printing(&self, depth: i32) {
        match self {
            AstTypeVariant::Primitive(primitive_type) => primitive_type.visit_for_printing(depth),
            AstTypeVariant::Array(array_type) => array_type.visit_for_printing(depth),
        }
    }
}

impl PrintUnparsedAST for AstTypeVariant {
    fn unparse_to_code(&self, depth: i32) {
        match self {
            AstTypeVariant::Primitive(primitive_type) => primitive_type.unparse_to_code(depth),
            AstTypeVariant::Array(array_type) => array_type.unparse_to_code(depth),
        }
    }
}