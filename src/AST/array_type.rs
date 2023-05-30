use std::fmt;
use std::sync::Arc;

use crate::ast::Ast;
use crate::ast::expression::ExprType;
use crate::ast::primitive_types::PrimitiveType;
use crate::utils::SourcePosition;

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

#[derive(Debug)]
pub enum TypeVariant {
    Primitive(PrimitiveType),
    Array(ArrayType),
}