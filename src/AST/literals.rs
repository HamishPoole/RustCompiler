use std::fmt;

use crate::ast::Ast;
use crate::utils::SourcePosition;

#[derive(Clone, Debug)]
pub struct Operator {
    pub source_position: SourcePosition,
    pub spelling: String,
}

impl Ast for Operator {
    fn visit(&self) {
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
    fn visit(&self) {
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
    fn visit(&self) {
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
    fn visit(&self) {
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
    fn visit(&self) {
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
    fn visit(&self) {
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

impl StringLiteral {
    pub fn new(source_position: SourcePosition, spelling: String) -> Self {
        Self {
            source_position,
            spelling,
        }
    }
}
