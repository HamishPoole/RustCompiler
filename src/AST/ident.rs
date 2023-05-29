use std::fmt;

use crate::ast::decl::DeclType;
use crate::ast::Ast;
use crate::utils::SourcePosition;

#[derive(Debug)]
pub struct Ident {
    value: String,
    source_position: SourcePosition,
    decl: Option<DeclType>,
}

impl Ast for Ident {
    fn visit(&self) {
        println!("Visiting Ident node.");
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Ident {
    pub fn new(value: String, source_position: SourcePosition, decl: Option<DeclType>) -> Self {
        Self {
            value,
            source_position,
            decl,
        }
    }
}