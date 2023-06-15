use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::ast::{Checking, PrintAST, PrintUnparsedAST};
use crate::ast::expression::ExprType;
use crate::ast::list::{DeclList, ListType, StmtList};
use crate::globals::TAB_SIZE;
use crate::utils::{generate_indent, generate_tabbed_string, print_indent, print_newline_and_indent, SourcePosition};

#[derive(Clone, Debug, PartialEq)]
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

impl PrintAST for StmtType {
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

impl PrintUnparsedAST for StmtType {
    fn unparse_to_code(&self, depth: i32) {
        match self {
            StmtType::BreakStmt(break_stmt) => break_stmt.unparse_to_code(depth)
            ,
            StmtType::CompoundStmt(compound_stmt) => compound_stmt.unparse_to_code(depth)
            ,
            StmtType::ContinueStmt(continue_stmt) => continue_stmt.unparse_to_code(depth)
            ,
            StmtType::EmptyCompoundStmt(empty_compound_stmt) => empty_compound_stmt.unparse_to_code(depth)
            ,
            StmtType::EmptyStmt(empty_stmt) => empty_stmt.unparse_to_code(depth)
            ,
            StmtType::ExprStmt(expr_stmt) => expr_stmt.unparse_to_code(depth)
            ,
            StmtType::ForStmt(for_stmt) => for_stmt.unparse_to_code(depth)
            ,
            StmtType::IfStmt(if_stmt) => if_stmt.unparse_to_code(depth)
            ,
            StmtType::ReturnStmt(return_stmt) => return_stmt.unparse_to_code(depth)
            ,
            StmtType::WhileStmt(while_stmt) => while_stmt.unparse_to_code(depth)
            ,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BreakStmt {
    source_position: SourcePosition,
}

impl Checking for BreakStmt {
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

impl PrintAST for BreakStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl PrintUnparsedAST for BreakStmt {
    fn unparse_to_code(&self, depth: i32) {
        let indent = generate_indent(depth);
        println!("{}break;", indent);
    }
}

impl BreakStmt {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompoundStmt {
    pub decl_list: Box<ListType>,
    pub stmt_list: Box<ListType>,
    pub source_position: SourcePosition,
}

impl Checking for CompoundStmt {
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

impl PrintAST for CompoundStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.decl_list.visit_for_printing(depth + 1);
        self.stmt_list.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for CompoundStmt {
    fn unparse_to_code(&self, depth: i32) {
        print!("{{");
        print_newline_and_indent(depth);

        self.decl_list.unparse_to_code(depth + 1);
        self.stmt_list.unparse_to_code(depth + 1);

        // Write the closing brace with the original indentation.
        print_newline_and_indent(depth);
        print!("}}");
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

#[derive(Clone, Debug, PartialEq)]
pub struct ContinueStmt {
    source_position: SourcePosition,
}

impl Checking for ContinueStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ContinueStmt node.");
    }
}

impl fmt::Display for ContinueStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ContinueStmt")
    }
}

impl PrintAST for ContinueStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl PrintUnparsedAST for ContinueStmt {
    fn unparse_to_code(&self, depth: i32) {
        println!("{}continue;", generate_indent(depth));
    }
}

impl ContinueStmt {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmptyStmt {
    source_position: SourcePosition,
}

impl Checking for EmptyStmt {
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

impl PrintAST for EmptyStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl PrintUnparsedAST for EmptyStmt {
    fn unparse_to_code(&self, depth: i32) {
        println!("{};", generate_indent(depth));
    }
}

impl EmptyStmt {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprStmt {
    source_position: SourcePosition,
    expr: ExprType,
}

impl Checking for ExprStmt {
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

impl PrintAST for ExprStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expr.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for ExprStmt {
    fn unparse_to_code(&self, depth: i32) {
        print_newline_and_indent(depth);
        self.expr.unparse_to_code(depth);
        print!(";");
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


#[derive(Clone, Debug, PartialEq)]
pub struct EmptyCompoundStmt {
    source_position: SourcePosition,
}

impl Checking for EmptyCompoundStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting EmptyCompoundStmt node.");
    }
}

impl fmt::Display for EmptyCompoundStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyCompoundStmt")
    }
}

impl PrintAST for EmptyCompoundStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl PrintUnparsedAST for EmptyCompoundStmt {
    fn unparse_to_code(&self, depth: i32) {
        println!("{}{{\n", generate_indent(depth));
        println!("{}}}", generate_indent(depth));
    }
}


impl EmptyCompoundStmt {
    pub fn new(source_position: SourcePosition) -> Self {
        Self {
            source_position,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForStmt {
    source_position: SourcePosition,
    expr_1: Box<ExprType>,
    expr_2: Box<ExprType>,
    expr_3: Box<ExprType>,
    stmt: Box<StmtType>,
}

impl Checking for ForStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ForStmt node.");
    }
}

impl fmt::Display for ForStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ForStmt")
    }
}

impl PrintAST for ForStmt {
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

impl PrintUnparsedAST for ForStmt {
    fn unparse_to_code(&self, depth: i32) {
        print_newline_and_indent(depth);
        print!("for (");
        self.expr_1.unparse_to_code(depth);
        print!("; ");
        self.expr_2.unparse_to_code(depth);
        print!("; ");
        self.expr_3.unparse_to_code(depth);
        print!(") ");
        let extra_depth = match *self.stmt {
            StmtType::CompoundStmt(_) => 0,
            _ => 1,
        };
        self.stmt.unparse_to_code(depth + extra_depth);
        print!("");
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

#[derive(Clone, Debug, PartialEq)]
pub struct IfStmt {
    source_position: SourcePosition,
    expr: Box<ExprType>,
    stmt_1: Box<StmtType>,
    stmt_2: Box<StmtType>,
}

impl Checking for IfStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IfStmt node.");
    }
}

impl fmt::Display for IfStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IfStmt")
    }
}

impl PrintAST for IfStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl PrintUnparsedAST for IfStmt {
    fn unparse_to_code(&self, depth: i32) {
        print_indent(depth);
        print!("if (");
        self.expr.unparse_to_code(depth);
        print!(") ");
        if let StmtType::CompoundStmt(_) = *self.stmt_1 {
            self.stmt_1.unparse_to_code(depth);
        } else {
            self.stmt_1.unparse_to_code(depth + 1);
        }

        match *self.stmt_2 {
            StmtType::EmptyStmt(_) => (),
            StmtType::IfStmt(_) => {
                print!("else");
                self.stmt_2.unparse_to_code(depth);
            }
            _ => {
                print_newline_and_indent(depth);
                print!("else");
                let extra_depth_s2 = match *self.stmt_2 {
                    StmtType::CompoundStmt(_) => 0,
                    _ => 1,
                };
                self.stmt_2.unparse_to_code(depth + extra_depth_s2);
            }
        }
        print!("");
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

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStmt {
    source_position: SourcePosition,
    expr: Box<ExprType>,
}

impl Checking for ReturnStmt {
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

impl PrintAST for ReturnStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expr.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for ReturnStmt {
    fn unparse_to_code(&self, depth: i32) {
        print_newline_and_indent(depth);
        print!("return");
        self.expr.unparse_to_code(depth);
        print!(";");
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

#[derive(Clone, Debug, PartialEq)]
pub struct WhileStmt {
    source_position: SourcePosition,
    expr: Box<ExprType>,
    stmt: Box<StmtType>,
}

impl Checking for WhileStmt {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting WhileStmt node.");
    }
}

impl fmt::Display for WhileStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WhileStmt")
    }
}

impl PrintAST for WhileStmt {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expr.visit_for_printing(depth);
        self.stmt.visit_for_printing(depth);
    }
}

impl PrintUnparsedAST for WhileStmt {
    fn unparse_to_code(&self, depth: i32) {
        print_newline_and_indent(depth);
        print!("while (");

        let extra_depth = match *self.stmt {
            StmtType::CompoundStmt(_) => 0,
            _ => 1,
        };

        self.expr.unparse_to_code(depth);
        println!(") ");
        self.stmt.unparse_to_code(depth + extra_depth);
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
