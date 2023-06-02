use std::any::{Any, TypeId};
use std::fmt;
use std::fmt::{Debug, Display};

use crate::ast::array_type::ArrayType;
use crate::ast::decl::{FuncDecl, GlobalVarDecl, LocalVarDecl, ParaDecl};
use crate::ast::expression::{
    Arg, ArrayExpr, ArrayInitExpr, AssignExpr, BinaryExpr, BooleanExpr, CallExpr, FloatExpr,
    IntExpr, StringExpr, UnaryExpr, VarExpr,
};
use crate::ast::ident::Ident;
use crate::ast::list::{ArrayExprList, DeclList, EmptyArgList, EmptyArrayExprList, EmptyParamList, ListType, ParamList, StmtList};
use crate::ast::literals::{
    BooleanLiteral, FloatLiteral, IntLiteral, Operator, StringLiteral, Terminal,
};
use crate::ast::primitive_types::{
    BooleanType, ErrorType, FloatType, IntType, StringType, VoidType,
};
use crate::ast::program::Program;
use crate::ast::statement::{
    BreakStmt, CompoundStmt, ContinueStmt, EmptyCompoundStmt, EmptyStmt, ExprStmt, ForStmt, IfStmt,
    ReturnStmt, WhileStmt,
};
use crate::ast::variable::VarUntyped;
use crate::globals::TAB_SIZE;
use crate::utils::SourcePosition;

pub mod decl;
pub mod expression;
pub mod ident;
pub mod list;
pub mod literals;
pub mod primitive_types;
pub mod program;
pub mod statement;
pub mod variable;
pub mod array_type;

pub trait Ast: Debug + Display {
    fn visit_for_semantics_checking(&self);
}

pub trait PrintingVisit {
    fn visit_for_printing(&self, depth: i32) {}
}


#[derive(Debug)]
pub enum AstNode {
    Arg(Arg),
    ArgList(ListType),
    ArrayExpr(ArrayExpr),
    ArrayExprList(ArrayExprList),
    ArrayInitExpr(ArrayInitExpr),
    ArrayType(ArrayType),
    AssignExpr(AssignExpr),
    BinaryExpr(BinaryExpr),
    BooleanExpr(BooleanExpr),
    BooleanLiteral(BooleanLiteral),
    BooleanType(BooleanType),
    BreakStmt(BreakStmt),
    CallExpr(CallExpr),
    CompoundStmt(CompoundStmt),
    ContinueStmt(ContinueStmt),
    DeclList(DeclList),
    EmptyArgList(EmptyArgList),
    EmptyArrayExprList(EmptyArrayExprList),
    EmptyCompoundStmt(EmptyCompoundStmt),
    EmptyDeclList(EmptyArgList),
    EmptyExpr,
    EmptyParamList(EmptyParamList),
    EmptyStmt(EmptyStmt),
    EmptyStmtList(EmptyArgList),
    ErrorType(ErrorType),
    ExprStmt(ExprStmt),
    FloatExpr(FloatExpr),
    FloatLiteral(FloatLiteral),
    FloatType(FloatType),
    ForStmt(ForStmt),
    FuncDecl(FuncDecl),
    GlobalVarDecl(GlobalVarDecl),
    Ident(Ident),
    IfStmt(IfStmt),
    IntExpr(IntExpr),
    IntLiteral(IntLiteral),
    IntType(IntType),
    LocalVarDecl(LocalVarDecl),
    Operator(Operator),
    ParaDecl(ParaDecl),
    ParamList(ParamList),
    Program(Program),
    ReturnStmt(ReturnStmt),
    StmtList(StmtList),
    StringExpr(StringExpr),
    StringLiteral(StringLiteral),
    StringType(StringType),
    Terminal(Terminal),
    UnaryExpr(UnaryExpr),
    Var(VarUntyped),
    VarExpr(VarExpr),
    VoidType(VoidType),
    WhileStmt(WhileStmt),
}

