use std::fmt;

use crate::ast::{Ast, PrintingVisit};
use crate::ast::ident::Ident;
use crate::ast::primitive_types::AstTypes;
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Clone, Debug)]
pub struct VarTyped {
    source_position: SourcePosition,
    var_type: Box<AstTypes>,
}

impl Ast for VarTyped {
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

impl PrintingVisit for VarTyped {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.var_type.visit_for_printing(depth + 1);
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


#[derive(Debug)]
pub struct VarUntyped {
    source_position: SourcePosition,
    ident: Ident,
}

impl Ast for VarUntyped {
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

impl PrintingVisit for VarUntyped {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.ident.visit_for_printing(depth + 1);
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
