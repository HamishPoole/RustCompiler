use std::fmt;

use crate::ast::expression::ExprType;
use crate::ast::list::{DeclList, ListType, StmtList};
use crate::ast::Ast;
use crate::utils::SourcePosition;

#[derive(Debug)]
pub enum StmtType {
    WhileStmt(WhileStmt),
    IfStmt(IfStmt),
    ExprStmt(ExprStmt),
    CompoundStmt(CompoundStmt),
    ForStmt(ForStmt),
    EmptyStmt(EmptyStmt),
    ContinueStmt(ContinueStmt),
    EmptyCompoundStmt(EmptyCompoundStmt),
    ReturnStmt(ReturnStmt),
    BreakStmt(BreakStmt),
}

#[derive(Debug)]
pub struct BreakStmt {
    source_position: SourcePosition,
}

impl Ast for BreakStmt {
    fn visit(&self) {
        println!("Visiting BreakStmt node.");
        // Implement visitBreakStmt function...
    }
}

impl fmt::Display for BreakStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "void")
    }
}

impl BreakStmt {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Debug)]
pub struct CompoundStmt {
    pub dl: Box<ListType>,
    pub sl: Box<ListType>,
    pub source_position: SourcePosition,
}

impl CompoundStmt {
    pub fn new(dl: Box<ListType>, sl: Box<ListType>, source_position: SourcePosition) -> Self {
        Self {
            dl,
            sl,
            source_position,
        }
    }
}

impl fmt::Display for CompoundStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CompoundStmt")
    }
}

#[derive(Debug)]
pub struct ContinueStmt {
    source_position: SourcePosition,
}

impl Ast for ContinueStmt {
    fn visit(&self) {
        println!("Visiting ContinueStmt node.");
    }
}

impl fmt::Display for ContinueStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ContinueStmt")
    }
}

impl ContinueStmt {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Debug)]
pub struct EmptyStmt {
    source_position: SourcePosition,
}

impl Ast for EmptyStmt {
    fn visit(&self) {
        println!("Visiting EmptyStmt node.");
        // Implement visitEmptyStmt function...
    }
}

impl fmt::Display for EmptyStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "void")
    }
}

impl EmptyStmt {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Debug)]
pub struct ExprStmt {
    source_position: SourcePosition,
    expr: ExprType,
}

impl Ast for ExprStmt {
    fn visit(&self) {
        println!("Visiting ExprStmt node.");
        // Implement visitExprStmt function...
    }
}

impl fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExprStmt")
    }
}

impl ExprStmt {
    pub fn new(source_position: SourcePosition, e: ExprType) -> Self {
        Self {
            source_position,
            expr: e,
        }
    }
}

#[derive(Debug)]
pub struct IfStmt {
    source_position: SourcePosition,
    e: Box<ExprType>,
    s1: Box<StmtType>,
    s2: Box<StmtType>,
}

impl Ast for IfStmt {
    fn visit(&self) {
        println!("Visiting IfStmt node.");
    }
}

impl fmt::Display for IfStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IfStmt")
    }
}

//
// #[derive(Debug)]
// pub struct CompoundStmt {
//     source_position: SourcePosition,
//     dl: DeclList,
//     sl: StmtList,
// }
//
// impl AST for CompoundStmt {
//     fn visit(&self) {
//         println!("Visiting CompoundStmt node.");
//     }
// }
//
// impl fmt::Display for CompoundStmt {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "CompoundStmt")
//     }
// }

#[derive(Debug)]
pub struct EmptyCompoundStmt {
    source_position: SourcePosition,
    dl: DeclList,
    sl: StmtList,
}

impl Ast for EmptyCompoundStmt {
    fn visit(&self) {
        println!("Visiting EmptyCompoundStmt node.");
    }
}

impl fmt::Display for EmptyCompoundStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyCompoundStmt")
    }
}

#[derive(Debug)]
pub struct ForStmt {
    source_position: SourcePosition,
    e1: Box<ExprType>,
    e2: Box<ExprType>,
    e3: Box<ExprType>,
    s: Box<StmtType>,
}

impl Ast for ForStmt {
    fn visit(&self) {
        println!("Visiting ForStmt node.");
    }
}

impl fmt::Display for ForStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ForStmt")
    }
}

#[derive(Debug)]
pub struct ReturnStmt {
    source_position: SourcePosition,
    expr: Box<ExprType>,
}

impl Ast for ReturnStmt {
    fn visit(&self) {
        println!("Visiting ReturnStmt node.");
        // Implement visitReturnStmt function...
    }
}

impl fmt::Display for ReturnStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "void")
    }
}

impl ReturnStmt {
    pub fn new(source_position: SourcePosition, expr: Box<ExprType>) -> Self {
        Self {
            source_position,
            expr,
        }
    }
}

#[derive(Debug)]
pub struct WhileStmt {
    source_position: SourcePosition,
    e: Box<ExprType>,
    s: Box<StmtType>,
}

impl Ast for WhileStmt {
    fn visit(&self) {
        println!("Visiting WhileStmt node.");
    }
}

impl fmt::Display for WhileStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WhileStmt")
    }
}

impl WhileStmt {
    pub fn new(source_position: SourcePosition, e: Box<ExprType>, s: Box<StmtType>) -> Self {
        Self {
            source_position,
            e,
            s,
        }
    }
}
