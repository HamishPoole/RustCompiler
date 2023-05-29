use std::fmt;

use crate::ast::expression::ExprType;
use crate::ast::ident::Ident;
use crate::ast::list::{ListType, ParamList};
use crate::ast::primitive_types::PrimitiveType;
use crate::ast::statement::StmtType;
use crate::ast::Ast;
use crate::utils::SourcePosition;

#[derive(Debug)]
pub enum DeclType {
    FuncDecl(FuncDecl),
    GlobalVarDecl(GlobalVarDecl),
    LocalVarDecl(LocalVarDecl),
    ParaDecl(ParaDecl),
}

#[derive(Debug)]
pub struct FuncDecl {
    source_position: SourcePosition,
    type_function: Box<PrimitiveType>,
    ident: Box<Ident>,
    param_list: Box<ListType>,
    statements: Box<StmtType>,
}

impl Ast for FuncDecl {
    fn visit(&self) {
        println!("Visiting FuncDecl node.");
        // Implement visitFuncDecl function...
    }
}

impl fmt::Display for FuncDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, t: {:?}, i: {:?}, pl: {:?}, s: {:?} }}",
            self.source_position, self.type_function, self.ident, self.param_list, self.statements
        )
    }
}

impl FuncDecl {
    pub fn new(
        source_position: SourcePosition,
        t: Box<PrimitiveType>,
        i: Box<Ident>,
        param_list: Box<ListType>,
        statement: Box<StmtType>,
    ) -> Self {
        Self {
            source_position,
            type_function: t,
            ident: i,
            param_list,
            statements: statement,
        }
    }
}

#[derive(Debug)]
pub struct GlobalVarDecl {
    source_position: SourcePosition,
    t: Box<PrimitiveType>,
    i: Box<Box<Ident>>,
    expr: Box<ExprType>,
}

impl fmt::Display for GlobalVarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, t: {:?}, i: {:?}, expr: {:?} }}",
            self.source_position, self.t, self.i, self.expr
        )
    }
}

impl Ast for GlobalVarDecl {
    fn visit(&self) {
        println!("Visiting GlobalVarDecl node.");
        // Implement visitGlobalVarDecl function...
    }
}

impl GlobalVarDecl {
    pub fn new(
        source_position: SourcePosition,
        t: Box<PrimitiveType>,
        i: Box<Box<Ident>>,
        expr: Box<ExprType>,
    ) -> Self {
        Self {
            source_position,
            t,
            i,
            expr,
        }
    }
}

#[derive(Debug)]
pub struct LocalVarDecl {
    source_position: SourcePosition,
    t: Box<PrimitiveType>,
    i: Box<Box<Ident>>,
    expr: Box<ExprType>,
}

impl fmt::Display for LocalVarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, t: {:?}, i: {:?}, expr: {:?} }}",
            self.source_position, self.t, self.i, self.expr
        )
    }
}

impl Ast for LocalVarDecl {
    fn visit(&self) {
        println!("Visiting LocalVarDecl node.");
        // Implement visitLocalVarDecl function...
    }
}

impl LocalVarDecl {
    pub fn new(
        source_position: SourcePosition,
        t: Box<PrimitiveType>,
        i: Box<Box<Ident>>,
        expr: Box<ExprType>,
    ) -> Self {
        Self {
            source_position,
            t,
            i,
            expr,
        }
    }
}

#[derive(Debug)]
pub struct ParaDecl {
    source_position: SourcePosition,
    t: Box<PrimitiveType>,
    i: Box<Ident>,
}

impl Ast for ParaDecl {
    fn visit(&self) {
        println!("Visiting ParaDecl node.");
        // Implement visitParaDecl function...
    }
}

impl fmt::Display for ParaDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, t: {:?}, i: {:?} }}",
            self.source_position, self.t, self.i
        )
    }
}

impl ParaDecl {
    pub fn new(source_position: SourcePosition, t: Box<PrimitiveType>, i: Box<Ident>) -> Self {
        Self {
            source_position,
            t,
            i,
        }
    }
}
