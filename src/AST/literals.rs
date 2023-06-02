use std::fmt;

use crate::ast::{Ast, PrintingVisit};
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Clone, Debug)]
pub struct Operator {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Ast for Operator {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintingVisit for Operator {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl Operator {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Terminal {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Ast for Terminal {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintingVisit for Terminal {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl Terminal {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntLiteral {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Ast for IntLiteral {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for IntLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintingVisit for IntLiteral {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl IntLiteral {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FloatLiteral {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Ast for FloatLiteral {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for FloatLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintingVisit for FloatLiteral {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl FloatLiteral {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BooleanLiteral {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Ast for BooleanLiteral {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for BooleanLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintingVisit for BooleanLiteral {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl BooleanLiteral {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StringLiteral {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Ast for StringLiteral {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.spelling
        )
    }
}

impl PrintingVisit for StringLiteral {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{} ({})", tabbed_string, self.spelling);
    }
}

impl StringLiteral {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}
