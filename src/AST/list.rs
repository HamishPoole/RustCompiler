use std::{fmt, println, todo};
use std::any::Any;

use crate::ast::{Ast, PrintingVisit};
use crate::ast::decl::{DeclType, ParaDecl};
use crate::ast::expression::{Arg, ExprType};
use crate::ast::statement::StmtType;
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Debug)]
pub enum ListType {
    ArgList(ArgList),
    DeclList(DeclList),
    EmptyArgList(EmptyArgList),
    EmptyDeclList(EmptyDeclList),
    EmptyArrayExprList(EmptyArrayExprList),
    ArrayExprList(ArrayExprList),
    EmptyStmtList(EmptyStmtList),
    StmtList(StmtList),
    ParamList(ParamList),
    EmptyParamList(EmptyParamList),
}

impl PrintingVisit for ListType {
    fn visit_for_printing(&self, depth: i32) {
        match self {
            ListType::ArgList(arg_list) => arg_list.visit_for_printing(depth),
            ListType::DeclList(decl_list) => decl_list.visit_for_printing(depth),
            ListType::EmptyArgList(empty_arg_list) => empty_arg_list.visit_for_printing(depth),
            ListType::EmptyDeclList(empty_decl_list) => empty_decl_list.visit_for_printing(depth),
            ListType::EmptyArrayExprList(empty_array_expr_list) => empty_array_expr_list.visit_for_printing(depth),
            ListType::ArrayExprList(array_expr_list) => array_expr_list.visit_for_printing(depth),
            ListType::EmptyStmtList(empty_stmt_list) => empty_stmt_list.visit_for_printing(depth),
            ListType::StmtList(stmt_list) => stmt_list.visit_for_printing(depth),
            ListType::ParamList(param_list) => param_list.visit_for_printing(depth),
            ListType::EmptyParamList(empty_param_list) => empty_param_list.visit_for_printing(depth),
        }
    }
}

#[derive(Debug)]
pub struct ArrayExprList {
    source_position: SourcePosition,
    expression: ExprType,
    expr_list: Box<ListType>,
}

impl Ast for ArrayExprList {
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

impl PrintingVisit for ArrayExprList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expression.visit_for_printing(depth + 1);
        self.expr_list.visit_for_printing(depth + 1);
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

#[derive(Debug)]
pub struct ArgList {
    source_position: SourcePosition,
    arg: ExprType,
    arg_list: Box<ListType>,
}


impl Ast for ArgList {
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

impl PrintingVisit for ArgList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.arg.visit_for_printing(depth + 1);
        self.arg_list.visit_for_printing(depth + 1);
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

#[derive(Debug)]
pub struct DeclList {
    source_position: SourcePosition,
    decl_type: Box<DeclType>,
    decl_list: Box<ListType>,
}

impl PrintingVisit for DeclList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.decl_type.visit_for_printing(depth + 1);
        self.decl_list.visit_for_printing(depth + 1);
    }
}

impl Ast for DeclList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting DeclList node.");
        todo!("Implement visitDeclList function in checker.rs")
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

#[derive(Debug)]
pub struct EmptyArgList {
    source_position: SourcePosition,
}


impl Ast for EmptyArgList {
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

impl PrintingVisit for EmptyArgList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl EmptyArgList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Debug)]
pub struct EmptyDeclList {
    source_position: SourcePosition,
}


impl Ast for EmptyDeclList {
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

impl PrintingVisit for EmptyDeclList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl EmptyDeclList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}


#[derive(Debug)]
pub struct EmptyArrayExprList {
    pub source_position: SourcePosition,
}


impl Ast for EmptyArrayExprList {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting EmptyArrayExprList node.");
        todo!("Implement visitEmptyArrayExprList function in checker.rs")
    }
}

impl std::fmt::Display for EmptyArrayExprList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ source_position: {:?} }}", self.source_position)
    }
}

impl PrintingVisit for EmptyArrayExprList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl EmptyArrayExprList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Debug)]
pub struct StmtList {
    source_position: SourcePosition,
    stmt: Box<StmtType>,
    stmt_list: Box<ListType>,
}


impl Ast for StmtList {
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

impl PrintingVisit for StmtList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.stmt.visit_for_printing(depth + 1);
        self.stmt_list.visit_for_printing(depth + 1);
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

#[derive(Debug)]
pub struct EmptyStmtList {
    source_position: SourcePosition,
}

impl PrintingVisit for EmptyStmtList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl Ast for EmptyStmtList {
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

impl EmptyStmtList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}

#[derive(Debug)]
pub struct ParamList {
    source_position: SourcePosition,
    param: ParaDecl,
    param_list: Box<ListType>,
}


impl Ast for ParamList {
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

impl PrintingVisit for ParamList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.param.visit_for_printing(depth + 1);
        self.param_list.visit_for_printing(depth + 1);
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

#[derive(Debug)]
pub struct EmptyParamList {
    source_position: SourcePosition,
}


impl Ast for EmptyParamList {
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

impl PrintingVisit for EmptyParamList {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl EmptyParamList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}
