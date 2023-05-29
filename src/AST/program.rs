use std::fmt;
use std::fmt::Display;

use crate::ast::list::DeclList;
use crate::utils::SourcePosition;

#[derive(Debug)]
struct Program {
    source_position: SourcePosition,
    declaration_list: Box<DeclList>,
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program")
    }
}

impl Program {
    fn new(source_position: SourcePosition, declaration_list: Box<DeclList>) -> Self {
        Self {
            source_position,
            declaration_list,
        }
    }
}
