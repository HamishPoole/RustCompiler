use std::fmt;

use crate::ast::{Ast, PrintingVisit};
use crate::ast::array_type::AstTypeVariant;
use crate::ast::expression::ExprType;
use crate::ast::ident::Ident;
use crate::ast::list::{ListType, ParamList};
use crate::ast::primitive_types::AstTypes;
use crate::ast::statement::StmtType;
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Debug)]
pub enum DeclType {
    FuncDecl(FuncDecl),
    GlobalVarDecl(GlobalVarDecl),
    LocalVarDecl(LocalVarDecl),
    ParaDecl(ParaDecl),
}

impl PrintingVisit for DeclType {
    fn visit_for_printing(&self, depth: i32) {
        match self {
            DeclType::FuncDecl(func_decl) => func_decl.visit_for_printing(depth ),
            DeclType::GlobalVarDecl(global_var_decl) => global_var_decl.visit_for_printing(depth ),
            DeclType::LocalVarDecl(local_var_decl) => local_var_decl.visit_for_printing(depth ),
            DeclType::ParaDecl(para_decl) => para_decl.visit_for_printing(depth ),
        }
    }
}

#[derive(Debug)]
pub struct FuncDecl {
    source_position: SourcePosition,
    function_type: Box<AstTypeVariant>,
    ident: Box<Ident>,
    param_list: Box<ListType>,
    statements: Box<StmtType>,
}

impl Ast for FuncDecl {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting FuncDecl node.");
        // Implement visitFuncDecl function...
    }
}

impl fmt::Display for FuncDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, t: {:?}, i: {:?}, pl: {:?}, s: {:?} }}",
            self.source_position, self.function_type, self.ident, self.param_list, self.statements
        )
    }
}

impl PrintingVisit for FuncDecl {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.function_type.visit_for_printing(depth + 1);
        self.ident.visit_for_printing(depth + 1);
        self.param_list.visit_for_printing(depth + 1);
        self.statements.visit_for_printing(depth + 1);
    }
}

impl FuncDecl {
    pub fn new(
        source_position: SourcePosition,
        function_type: Box<AstTypeVariant>,
        identifier: Box<Ident>,
        param_list: Box<ListType>,
        statement: Box<StmtType>,
    ) -> Self {
        Self {
            source_position,
            function_type,
            ident: identifier,
            param_list,
            statements: statement,
        }
    }
}

#[derive(Debug)]
pub struct GlobalVarDecl {
    source_position: SourcePosition,
    t: Box<AstTypeVariant>,
    i: Box<Ident>,
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
    fn visit_for_semantics_checking(&self) {
        println!("Visiting GlobalVarDecl node.");
        // Implement visitGlobalVarDecl function...
    }
}

impl PrintingVisit for GlobalVarDecl {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.t.visit_for_printing(depth + 1);
        self.i.visit_for_printing(depth + 1);
        self.expr.visit_for_printing(depth + 1);
    }
}

impl GlobalVarDecl {
    pub fn new(
        source_position: SourcePosition,
        t: Box<AstTypeVariant>,
        i: Box<Ident>,
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
    declaration_type: Box<AstTypeVariant>,
    ident: Box<Ident>,
    expr: Box<ExprType>,
}

impl fmt::Display for LocalVarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, t: {:?}, i: {:?}, expr: {:?} }}",
            self.source_position, self.declaration_type, self.ident, self.expr
        )
    }
}

impl Ast for LocalVarDecl {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting LocalVarDecl node.");
        // Implement visitLocalVarDecl function...
    }
}

impl PrintingVisit for LocalVarDecl {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.declaration_type.visit_for_printing(depth + 1);
        self.ident.visit_for_printing(depth + 1);
        self.expr.visit_for_printing(depth + 1);
    }
}

impl LocalVarDecl {
    pub fn new(
        source_position: SourcePosition,
        declaration_type: Box<AstTypeVariant>,
        ident: Box<Ident>,
        expr: Box<ExprType>,
    ) -> Self {
        Self {
            source_position,
            declaration_type,
            ident,
            expr,
        }
    }
}

#[derive(Debug)]
pub struct ParaDecl {
    source_position: SourcePosition,
    decl_type: Box<AstTypeVariant>,
    ident: Box<Ident>,
}

impl Ast for ParaDecl {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ParaDecl node.");
        // Implement visitParaDecl function...
    }
}

impl fmt::Display for ParaDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, t: {:?}, i: {:?} }}",
            self.source_position, self.decl_type, self.ident
        )
    }
}

impl PrintingVisit for ParaDecl {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.decl_type.visit_for_printing(depth + 1);
        self.ident.visit_for_printing(depth + 1);
    }
}

impl ParaDecl {
    pub fn new(source_position: SourcePosition, decl_type: Box<AstTypeVariant>, ident: Box<Ident>) -> Self {
        Self {
            source_position,
            decl_type,
            ident,
        }
    }
}