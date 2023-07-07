use std::{fmt, println, todo};
use std::any::Any;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::ast::{Checking, PrintAST, PrintUnparsedAST};
use crate::ast::decl::{DeclType, ParaDecl};
use crate::ast::expression::{Arg, ExprType};
use crate::ast::statement::StmtType;
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Clone, Debug, PartialEq)]
pub enum ListType {
    ArgList(ArgList),
    ArrayExprList(ArrayExprList),
    DeclList(DeclList),
    EmptyArgList(EmptyArgList),
    EmptyArrayExprList(EmptyArrayExprList),
    EmptyDeclList(EmptyDeclList),
    EmptyParamList(EmptyParamList),
    EmptyStmtList(EmptyStmtList),
    ParamList(ParamList),
    StmtList(StmtList),
}

impl PrintAST for ListType {
    fn visit_for_printing(&self, depth: i32) {
        match self {
            ListType::ArgList(arg_list) => arg_list.visit_for_printing(depth),
            ListType::ArrayExprList(array_expr_list) => array_expr_list.visit_for_printing(depth),
            ListType::DeclList(decl_list) => decl_list.visit_for_printing(depth),
            ListType::EmptyArrayExprList(empty_array_expr_list) => empty_array_expr_list.visit_for_printing(depth),
            ListType::EmptyArgList(empty_arg_list) => empty_arg_list.visit_for_printing(depth),
            ListType::EmptyDeclList(empty_decl_list) => empty_decl_list.visit_for_printing(depth),
            ListType::EmptyParamList(empty_param_list) => empty_param_list.visit_for_printing(depth),
            ListType::EmptyStmtList(empty_stmt_list) => empty_stmt_list.visit_for_printing(depth),
            ListType::StmtList(stmt_list) => stmt_list.visit_for_printing(depth),
            ListType::ParamList(param_list) => param_list.visit_for_printing(depth),
        }
    }
}

impl PrintUnparsedAST for ListType {
    fn unparse_to_code(&self, depth: i32) {
        match self {
            ListType::ArgList(arg_list) => arg_list.unparse_to_code(depth),
            ListType::DeclList(decl_list) => decl_list.unparse_to_code(depth),
            ListType::EmptyArgList(empty_arg_list) => empty_arg_list.unparse_to_code(depth),
            ListType::EmptyDeclList(empty_decl_list) => empty_decl_list.unparse_to_code(depth),
            ListType::EmptyArrayExprList(empty_array_expr_list) => empty_array_expr_list.unparse_to_code(depth),
            ListType::ArrayExprList(array_expr_list) => array_expr_list.unparse_to_code(depth),
            ListType::EmptyStmtList(empty_stmt_list) => empty_stmt_list.unparse_to_code(depth),
            ListType::StmtList(stmt_list) => stmt_list.unparse_to_code(depth),
            ListType::ParamList(param_list) => param_list.unparse_to_code(depth),
            ListType::EmptyParamList(empty_param_list) => empty_param_list.unparse_to_code(depth),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayExprList {
    source_position: SourcePosition,
    expression: ExprType,
    expr_list: Box<ListType>,
}

impl Checking for ArrayExprList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ArrayExprList node.");
        // Implement visitArrayExprList function...
    }
}

impl fmt::Display for ArrayExprList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayExprList")
    }
}

impl PrintAST for ArrayExprList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expression.visit_for_printing(depth + 1);
        self.expr_list.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for ArrayExprList {
    fn unparse_to_code(&self, depth: i32) {
        self.expression.unparse_to_code(depth);
        if !matches!(*self.expr_list, ListType::EmptyArrayExprList(_)) {
            println!(", ");
        }

        self.expr_list.unparse_to_code(depth);
    }
}

impl ArrayExprList {
    pub fn new(
        source_position: SourcePosition,
        expression: ExprType,
        expr_list: Box<ListType>,
    ) -> Self {
        Self {
            source_position,
            expression,
            expr_list,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArgList {
    source_position: SourcePosition,
    arg: ExprType,
    arg_list: Box<ListType>,
}

impl Checking for ArgList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ArgList node.");
        // Implement visitArgList function...
    }
}

impl fmt::Display for ArgList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArgList")
    }
}

impl PrintAST for ArgList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.arg.visit_for_printing(depth + 1);
        self.arg_list.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for ArgList {
    fn unparse_to_code(&self, depth: i32) {
        // Print nothing for the program stage.
        self.arg.unparse_to_code(depth + 1);
        if !matches!(*self.arg_list, ListType::EmptyArgList(_)) {
            println!(", ");
        }

        self.arg_list.unparse_to_code(depth + 1);
    }
}

impl ArgList {
    pub fn new(source_position: SourcePosition, arg: ExprType, arg_list: Box<ListType>) -> Self {
        Self {
            source_position,
            arg,
            arg_list,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclList {
    source_position: SourcePosition,
    decl_type: Box<DeclType>,
    decl_list: Box<ListType>,
}

impl PrintAST for DeclList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.decl_type.visit_for_printing(depth + 1);
        self.decl_list.visit_for_printing(depth + 1);
    }
}

impl Checking for DeclList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting DeclList node.");
        todo!("Implement visitDeclList function in checker.rs")
    }
}

impl PrintUnparsedAST for DeclList {
    fn unparse_to_code(&self, depth: i32) {
        self.decl_type.unparse_to_code(depth);
        self.decl_list.unparse_to_code(depth);
    }
}

impl fmt::Display for DeclList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{source_position: {:?}, d: {:?}, dl: {:?} }}",
            self.source_position, self.decl_type, self.decl_list
        )
    }
}

impl DeclList {
    pub fn new(source_position: SourcePosition, decl_type: Box<DeclType>, decl_list: Box<ListType>) -> Self {
        Self {
            source_position,
            decl_type,
            decl_list,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmptyArgList {
    source_position: SourcePosition,
}

impl Checking for EmptyArgList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting EmptyArgList node.");
        // Implement visitEmptyArgList function...
    }
}

impl fmt::Display for EmptyArgList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyArgList")
    }
}

impl PrintAST for EmptyArgList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl PrintUnparsedAST for EmptyArgList {
    fn unparse_to_code(&self, depth: i32) {
        print!(")")
    }
}

impl EmptyArgList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmptyDeclList {
    source_position: SourcePosition,
}

impl Checking for EmptyDeclList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting EmptyDeclList node.");
        // Implement visitEmptyDeclList function...
    }
}

impl fmt::Display for EmptyDeclList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyDeclList")
    }
}

impl PrintAST for EmptyDeclList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl PrintUnparsedAST for EmptyDeclList {
    fn unparse_to_code(&self, depth: i32) {}
}

impl EmptyDeclList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmptyArrayExprList {
    pub source_position: SourcePosition,
}

impl Checking for EmptyArrayExprList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting EmptyArrayExprList node.");
    }
}

impl std::fmt::Display for EmptyArrayExprList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ source_position: {:?} }}", self.source_position)
    }
}

impl PrintAST for EmptyArrayExprList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}


impl PrintUnparsedAST for EmptyArrayExprList {
    fn unparse_to_code(&self, depth: i32) {}
}

impl EmptyArrayExprList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtList {
    source_position: SourcePosition,
    stmt: Box<StmtType>,
    stmt_list: Box<ListType>,
}

impl Checking for StmtList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting StmtList node.");
        // Implement visitStmtList function...
    }
}

impl fmt::Display for StmtList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StmtList")
    }
}

impl PrintAST for StmtList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.stmt.visit_for_printing(depth + 1);
        self.stmt_list.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for StmtList {
    fn unparse_to_code(&self, depth: i32) {
        self.stmt.unparse_to_code(depth);
        self.stmt_list.unparse_to_code(depth);
    }
}

impl StmtList {
    pub fn new(
        source_position: SourcePosition,
        stmt: Box<StmtType>,
        stmt_list: Box<ListType>,
    ) -> Self {
        Self {
            source_position,
            stmt,
            stmt_list,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmptyStmtList {
    source_position: SourcePosition,
}

impl PrintAST for EmptyStmtList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl Checking for EmptyStmtList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting EmptyStmtList node.");
        // Implement visitEmptyStmtList function...
    }
}

impl fmt::Display for EmptyStmtList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyStmtList")
    }
}

impl PrintUnparsedAST for EmptyStmtList {
    fn unparse_to_code(&self, depth: i32) {}
}

impl EmptyStmtList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParamList {
    source_position: SourcePosition,
    param: ParaDecl,
    param_list: Box<ListType>,
}

impl Checking for ParamList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ParamList node.");
        // Implement visitParamList function...
    }
}

impl fmt::Display for ParamList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParamList")
    }
}

impl PrintAST for ParamList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.param.visit_for_printing(depth + 1);
        self.param_list.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for ParamList {
    fn unparse_to_code(&self, depth: i32) {
        self.param.unparse_to_code(depth);
        if !matches!(*self.param_list, ListType::EmptyParamList(_)) {
            println!(", ");
        }
        self.param_list.unparse_to_code(depth);
    }
}

impl ParamList {
    pub fn new(
        source_position: SourcePosition,
        param: ParaDecl,
        param_list: Box<ListType>,
    ) -> Self {
        Self {
            source_position,
            param,
            param_list,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EmptyParamList {
    source_position: SourcePosition,
}

impl Checking for EmptyParamList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting EmptyParamList node.");
        // Implement visitEmptyParamList function...
    }
}

impl fmt::Display for EmptyParamList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyParamList")
    }
}

impl PrintAST for EmptyParamList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl PrintUnparsedAST for EmptyParamList {
    fn unparse_to_code(&self, depth: i32) {
        println!(")");
    }
}

impl EmptyParamList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}
