use std::fmt;
use std::sync::Arc;

use crate::ast::{Ast, PrintingVisit};
use crate::ast::expression::ExprType;
use crate::ast::primitive_types::AstTypes;
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Debug)]
pub struct ArrayType {
    source_position: SourcePosition,
    array_type: Arc<Box<AstTypes>>,
    expression: ExprType,
}

impl Ast for ArrayType {
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

impl PrintingVisit for ArrayType {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.array_type.visit_for_printing(depth + 1);
        self.expression.visit_for_printing(depth + 1);
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

#[derive(Debug)]
pub enum AstTypeVariant {
    Primitive(AstTypes),
    Array(ArrayType),
}

impl PrintingVisit for AstTypeVariant {
    fn visit_for_printing(&self, depth: i32) {
        match self {
            AstTypeVariant::Primitive(primitive_type) => primitive_type.visit_for_printing(depth),
            AstTypeVariant::Array(array_type) => array_type.visit_for_printing(depth),
        }
    }
}