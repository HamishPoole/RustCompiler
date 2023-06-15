use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::ast::{Checking, PrintAST, PrintUnparsedAST};
use crate::ast::decl::DeclType;
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Clone, Debug, PartialEq)]
pub struct Ident {
    pub spelling: String,
    pub source_position: SourcePosition,
    pub decl: Option<DeclType>,
}

impl Checking for Ident {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting Ident node.");
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.spelling)
    }
}

impl PrintAST for Ident {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl PrintUnparsedAST for Ident {
    fn unparse_to_code(&self, depth: i32) {
        print!("{}", self.spelling);
    }
}

impl Ident {
    pub fn new(spelling: String, source_position: SourcePosition, decl: Option<DeclType>) -> Self {
        Self {
            spelling,
            source_position,
            decl,
        }
    }

    pub fn copy_with_null_decl(&self) -> Self {
        Self {
            spelling: self.spelling.clone(),
            source_position: self.source_position,
            decl: None,
        }
    }
}
