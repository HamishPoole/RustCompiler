use std::fmt;

use crate::ast::{Ast, PrintingVisit};
use crate::ast::expression::ExprType;
use crate::ast::list::{DeclList, ListType, StmtList};
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Debug)]
pub enum StmtType {
    BreakStmt(BreakStmt),
    CompoundStmt(CompoundStmt),
    ContinueStmt(ContinueStmt),
    EmptyCompoundStmt(EmptyCompoundStmt),
    EmptyStmt(EmptyStmt),
    ExprStmt(ExprStmt),
    ForStmt(ForStmt),
    IfStmt(IfStmt),
    ReturnStmt(ReturnStmt),
    WhileStmt(WhileStmt),
}

impl PrintingVisit for StmtType {
    fn visit_for_printing(&self, depth: i32) {
        match self {
            StmtType::BreakStmt(break_stmt) => break_stmt.visit_for_printing(depth),
            StmtType::CompoundStmt(compound_stmt) => compound_stmt.visit_for_printing(depth),
            StmtType::ContinueStmt(continue_stmt) => continue_stmt.visit_for_printing(depth),
            StmtType::EmptyCompoundStmt(empty_compound_stmt) => empty_compound_stmt.visit_for_printing(depth),
            StmtType::EmptyStmt(empty_stmt) => empty_stmt.visit_for_printing(depth),
            StmtType::ExprStmt(expr_stmt) => expr_stmt.visit_for_printing(depth),
            StmtType::ForStmt(for_stmt) => for_stmt.visit_for_printing(depth),
            StmtType::IfStmt(if_stmt) => if_stmt.visit_for_printing(depth),
            StmtType::ReturnStmt(return_stmt) => return_stmt.visit_for_printing(depth),
            StmtType::WhileStmt(while_stmt) => while_stmt.visit_for_printing(depth),
        }
    }
}

#[derive(Debug)]
pub struct BreakStmt {
    source_position: SourcePosition,
}

impl Ast for BreakStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting BreakStmt node.");
        // Implement visitBreakStmt function...
    }
}

impl fmt::Display for BreakStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "void")
    }
}


impl PrintingVisit for BreakStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl BreakStmt {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Debug)]
pub struct CompoundStmt {
    pub decl_list: Box<ListType>,
    pub stmt_list: Box<ListType>,
    pub source_position: SourcePosition,
}

impl Ast for CompoundStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting CompoundStmt node.");
        // Implement visitCompoundStmt function...
    }
}

impl fmt::Display for CompoundStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CompoundStmt")
    }
}


impl PrintingVisit for CompoundStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.decl_list.visit_for_printing(depth + 1);
        self.stmt_list.visit_for_printing(depth + 1);
    }
}

impl CompoundStmt {
    pub fn new(decl_list: Box<ListType>, stmt_list: Box<ListType>, source_position: SourcePosition) -> Self {
        Self {
            decl_list,
            stmt_list,
            source_position,
        }
    }
}


#[derive(Debug)]
pub struct ContinueStmt {
    source_position: SourcePosition,
}

impl Ast for ContinueStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ContinueStmt node.");
    }
}

impl fmt::Display for ContinueStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ContinueStmt")
    }
}

impl PrintingVisit for ContinueStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
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
    fn visit_for_semantics_checking(&self) {
        println!("Visiting EmptyStmt node.");
        // Implement visitEmptyStmt function...
    }
}

impl fmt::Display for EmptyStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "void")
    }
}

impl PrintingVisit for EmptyStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
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
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ExprStmt node.");
        // Implement visitExprStmt function...
    }
}

impl fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExprStmt")
    }
}

impl PrintingVisit for ExprStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expr.visit_for_printing(depth + 1);
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
    expr: Box<ExprType>,
    stmt_1: Box<StmtType>,
    stmt_2: Box<StmtType>,
}

impl Ast for IfStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IfStmt node.");
    }
}

impl fmt::Display for IfStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IfStmt")
    }
}

impl PrintingVisit for IfStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl IfStmt {
    pub fn new(source_position: SourcePosition, expr: Box<ExprType>, stmt_1: Box<StmtType>, stmt_2: Box<StmtType>) -> Self {
        Self {
            source_position,
            expr,
            stmt_1,
            stmt_2,
        }
    }
}

#[derive(Debug)]
pub struct EmptyCompoundStmt {
    source_position: SourcePosition,
}

impl Ast for EmptyCompoundStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting EmptyCompoundStmt node.");
    }
}

impl fmt::Display for EmptyCompoundStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyCompoundStmt")
    }
}

impl PrintingVisit for EmptyCompoundStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl EmptyCompoundStmt {
    pub fn new(source_position: SourcePosition) -> Self {
        Self {
            source_position,
        }
    }
}

#[derive(Debug)]
pub struct ForStmt {
    source_position: SourcePosition,
    expr_1: Box<ExprType>,
    expr_2: Box<ExprType>,
    expr_3: Box<ExprType>,
    stmt: Box<StmtType>,
}

impl Ast for ForStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ForStmt node.");
    }
}

impl fmt::Display for ForStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ForStmt")
    }
}

impl PrintingVisit for ForStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expr_1.visit_for_printing(depth + 1);
        self.expr_2.visit_for_printing(depth + 1);
        self.expr_3.visit_for_printing(depth + 1);
        self.stmt.visit_for_printing(depth + 1);
    }
}

impl ForStmt {
    pub fn new(source_position: SourcePosition, expr_1: Box<ExprType>, expr_2: Box<ExprType>, expr_3: Box<ExprType>, stmt: Box<StmtType>) -> Self {
        Self {
            source_position,
            expr_1,
            expr_2,
            expr_3,
            stmt,
        }
    }
}

#[derive(Debug)]
pub struct ReturnStmt {
    source_position: SourcePosition,
    expr: Box<ExprType>,
}

impl Ast for ReturnStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ReturnStmt node.");
        // Implement visitReturnStmt function...
    }
}

impl fmt::Display for ReturnStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "void")
    }
}

impl PrintingVisit for ReturnStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expr.visit_for_printing(depth + 1);
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
    expr: Box<ExprType>,
    stmt: Box<StmtType>,
}

impl Ast for WhileStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting WhileStmt node.");
    }
}

impl fmt::Display for WhileStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WhileStmt")
    }
}

impl PrintingVisit for WhileStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expr.visit_for_printing(depth + 1);
        self.stmt.visit_for_printing(depth + 1);
    }
}

impl WhileStmt {
    pub fn new(source_position: SourcePosition, expr: Box<ExprType>, stmt: Box<StmtType>) -> Self {
        Self {
            source_position,
            expr,
            stmt,
        }
    }
}
