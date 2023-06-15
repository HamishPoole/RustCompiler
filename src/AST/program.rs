use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::BufWriter;

use crate::ast::{Checking, PrintAST, PrintUnparsedAST};
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

impl Checking for Program {
    fn visit_for_semantics_checking(&self) {
        todo!("Implement visit_for_semantics_checking function...")
    }
}

impl PrintAST for Program {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.declaration_list.visit_for_printing(depth + 1);
    }
}


impl PrintUnparsedAST for Program {
    fn unparse_to_code(&self, depth: i32) {
        // Print nothing for the program stage.
        self.declaration_list.unparse_to_code(depth);
    }
}

impl Program {
    pub fn new(declaration_list: DeclList) -> Self {
        Self {
            declaration_list,
        }
    }


    pub fn print_program(&self) {
        self.visit_for_printing(0);
    }

    pub fn print_unparsed_program(&self) {
        self.unparse_to_code(0);
    }
}
