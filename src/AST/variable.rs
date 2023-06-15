use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::ast::{Checking, PrintAST, PrintUnparsedAST};
use crate::ast::ident::Ident;
use crate::ast::primitive_types::AstTypes;
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Clone, Debug, PartialEq)]
pub struct VarTyped {
    source_position: SourcePosition,
    var_type: Box<AstTypes>,
}

impl Checking for VarTyped {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting Var node.");
        // Implement visitVar function...
    }
}

impl fmt::Display for VarTyped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VarTyped")
    }
}

impl PrintAST for VarTyped {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.var_type.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for VarTyped {
    fn unparse_to_code(&self, depth: i32) {
        self.var_type.unparse_to_code(depth);
    }
}

impl VarTyped {
    pub fn new(source_position: SourcePosition, var_type: Box<AstTypes>) -> Self {
        Self {
            source_position,
            var_type,
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct VarUntyped {
    source_position: SourcePosition,
    ident: Ident,
}

impl Checking for VarUntyped {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting Var node.");
        // Implement visitVar function...
    }
}

impl fmt::Display for VarUntyped {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VarUntyped")
    }
}

impl PrintAST for VarUntyped {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.ident.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for VarUntyped {
    fn unparse_to_code(&self, depth: i32) {
        self.ident.unparse_to_code(depth)
    }
}

impl VarUntyped {
    pub fn new(source_position: SourcePosition, ident: Ident) -> Self {
        Self {
            source_position,
            ident,
        }
    }
}
