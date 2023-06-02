use std::fmt;
use std::fmt::Display;

use crate::ast::{Ast, PrintingVisit};
use crate::ast::list::DeclList;
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Debug)]
pub struct Program {
    declaration_list: DeclList,
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program")
    }
}

impl Ast for Program {
    fn visit_for_semantics_checking(&self) {
        todo!("Implement visit_for_semantics_checking function...")
    }
}

impl PrintingVisit for Program {}

impl Program {
    pub fn new(declaration_list: DeclList) -> Self {
        Self {
            declaration_list,
        }
    }

    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.declaration_list.visit_for_printing(depth + 1);
    }

    pub fn print_program(&self) {
        self.visit_for_printing(0);
    }
}
