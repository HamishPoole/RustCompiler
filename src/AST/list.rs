use std::any::Any;
use std::{fmt, println, todo};

use crate::ast::decl::{DeclType, ParaDecl};
use crate::ast::expression::ExprType;
use crate::ast::statement::StmtType;
use crate::ast::Ast;
use crate::utils::SourcePosition;

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

#[derive(Debug)]
pub struct ArrayExprList {
    source_position: SourcePosition,
    expression: ExprType,
    expr_list: Box<ListType>,
}

impl Ast for ArrayExprList {
    fn visit(&self) {
        println!("Visiting ArrayExprList node.");
        // Implement visitArrayExprList function...
    }
}

impl fmt::Display for ArrayExprList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayExprList")
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
    fn visit(&self) {
        println!("Visiting ArgList node.");
        // Implement visitArgList function...
    }
}

impl fmt::Display for ArgList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArgList")
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
    d: Box<DeclType>,
    dl: Box<DeclList>,
}

impl Ast for DeclList {
    fn visit(&self) {
        println!("Visiting DeclList node.");
        todo!("Implement visitDeclList function in checker.rs")
    }
}

impl fmt::Display for DeclList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{source_position: {:?}, d: {:?}, dl: {:?} }}",
            self.source_position, self.d, self.dl
        )
    }
}

impl DeclList {
    pub fn new(source_position: SourcePosition, d: Box<DeclType>, dl: Box<DeclList>) -> Self {
        Self {
            source_position,
            d,
            dl,
        }
    }
}

#[derive(Debug)]
pub struct EmptyArgList {
    source_position: SourcePosition,
}

impl Ast for EmptyArgList {
    fn visit(&self) {
        println!("Visiting EmptyArgList node.");
        // Implement visitEmptyArgList function...
    }
}

impl fmt::Display for EmptyArgList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyArgList")
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
    fn visit(&self) {
        println!("Visiting EmptyDeclList node.");
        // Implement visitEmptyDeclList function...
    }
}

impl fmt::Display for EmptyDeclList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyDeclList")
    }
}

impl EmptyDeclList {
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
    fn visit(&self) {
        println!("Visiting StmtList node.");
        // Implement visitStmtList function...
    }
}

#[derive(Debug)]
pub struct EmptyArrayExprList {
    pub source_position: SourcePosition,
}

impl Ast for EmptyArrayExprList {
    fn visit(&self) {
        println!("Visiting EmptyArrayExprList node.");
        todo!("Implement visitEmptyArrayExprList function in checker.rs")
    }
}

impl std::fmt::Display for EmptyArrayExprList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ source_position: {:?} }}", self.source_position)
    }
}

impl EmptyArrayExprList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
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

impl fmt::Display for StmtList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StmtList")
    }
}

#[derive(Debug)]
pub struct EmptyStmtList {
    source_position: SourcePosition,
}

impl Ast for EmptyStmtList {
    fn visit(&self) {
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
    fn visit(&self) {
        println!("Visiting ParamList node.");
        // Implement visitParamList function...
    }
}

impl fmt::Display for ParamList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParamList")
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
    fn visit(&self) {
        println!("Visiting EmptyParamList node.");
        // Implement visitEmptyParamList function...
    }
}

impl fmt::Display for EmptyParamList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EmptyParamList")
    }
}

impl EmptyParamList {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}
