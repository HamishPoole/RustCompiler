use std::any::{Any, TypeId};
use std::fmt;
use std::fmt::{Debug, Display};

use crate::ast::decl::{FuncDecl, GlobalVarDecl, LocalVarDecl, ParaDecl};
use crate::ast::expression::{
    Arg, ArrayExpr, ArrayInitExpr, AssignExpr, BinaryExpr, BooleanExpr, CallExpr, FloatExpr,
    IntExpr, StringExpr, UnaryExpr, VarExpr,
};
use crate::ast::ident::Ident;
use crate::ast::list::{ArgList, ArrayExprList, DeclList, EmptyArgList, EmptyArrayExprList, EmptyParamList, ListType, ParamList, StmtList};
use crate::ast::literals::{
    BooleanLiteral, FloatLiteral, IntLiteral, Operator, StringLiteral, Terminal,
};
use crate::ast::primitive_types::{
    ArrayType, BooleanType, ErrorType, FloatType, IntType, StringType, VoidType,
};
use crate::ast::statement::{
    BreakStmt, CompoundStmt, ContinueStmt, EmptyCompoundStmt, EmptyStmt, ExprStmt, ForStmt, IfStmt,
    ReturnStmt, WhileStmt,
};
use crate::ast::variable::Var;
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
    fn visit(&self);
}
//
// #[derive(Debug)]
// pub struct AstNode {
//     source_position: SourcePosition,
// }
//
// impl fmt::Display for AstNode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "AstNode")
//     }
// }
//
// impl AST for AstNode {
//     fn visit(&self) {
//         println!("Visiting AST_root node.");
//         // Implement visitAST_root function...
//
// }

#[derive(Debug)]
pub enum AstNode {
    Arg(Arg),
    ArgList(ArgList),
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
    Var(Var),
    VarExpr(VarExpr),
    VoidType(VoidType),
    WhileStmt(WhileStmt),
}

#[derive(Debug)]
pub struct Program {
    pub decl_list: ListType,
}




