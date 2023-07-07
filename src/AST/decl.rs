use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::ast::array_type::{ArrayType, AstTypeVariant};
use crate::ast::expression::ExprType;
use crate::ast::ident::Ident;
use crate::ast::list::{ListType, ParamList};
use crate::ast::primitive_types::AstTypes;
use crate::ast::statement::StmtType;
use crate::ast::{Checking, PrintAST, PrintUnparsedAST};
use crate::globals::TAB_SIZE;
use crate::utils::{
    generate_tabbed_string, print_indent, print_newline_and_indent, SourcePosition,
};

#[derive(Clone, Debug, PartialEq)]
pub enum DeclType {
    FuncDecl(FuncDecl),
    GlobalVarDecl(GlobalVarDecl),
    LocalVarDecl(LocalVarDecl),
    ParaDecl(ParaDecl),
}

impl PrintAST for DeclType {
    fn visit_for_printing(&self, depth: i32) {
        match self {
            DeclType::FuncDecl(func_decl) => func_decl.visit_for_printing(depth),
            DeclType::GlobalVarDecl(global_var_decl) => global_var_decl.visit_for_printing(depth),
            DeclType::LocalVarDecl(local_var_decl) => local_var_decl.visit_for_printing(depth),
            DeclType::ParaDecl(para_decl) => para_decl.visit_for_printing(depth),
        }
    }
}

impl PrintUnparsedAST for DeclType {
    fn unparse_to_code(&self, depth: i32) {
        match self {
            DeclType::FuncDecl(func_decl) => func_decl.unparse_to_code(depth),
            DeclType::GlobalVarDecl(global_var_decl) => global_var_decl.unparse_to_code(depth),
            DeclType::LocalVarDecl(local_var_decl) => local_var_decl.unparse_to_code(depth),
            DeclType::ParaDecl(para_decl) => para_decl.unparse_to_code(depth),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FuncDecl {
    source_position: SourcePosition,
    function_type: Box<AstTypeVariant>,
    ident: Box<Ident>,
    param_list: Box<ListType>,
    statements: Box<StmtType>,
}

impl Checking for FuncDecl {
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

impl PrintAST for FuncDecl {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.function_type.visit_for_printing(depth + 1);
        self.ident.visit_for_printing(depth + 1);
        self.param_list.visit_for_printing(depth + 1);
        self.statements.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for FuncDecl {
    fn unparse_to_code(&self, depth: i32) {
        // If line start and char start are 1, print newline.
        // Else, print indent
        if self.source_position.line_start == 1 && self.source_position.char_start == 1 {
            print_newline_and_indent(depth);
        } else {
            print_indent(depth);
        }
        self.function_type.unparse_to_code(depth);
        print!(" ");
        self.ident.unparse_to_code(depth);
        print!("(");
        self.param_list.unparse_to_code(depth);
        self.statements.unparse_to_code(depth);
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

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalVarDecl {
    source_position: SourcePosition,
    declaration_type: Box<AstTypeVariant>,
    ident: Box<Ident>,
    expr: Box<ExprType>,
}

impl fmt::Display for GlobalVarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, t: {:?}, i: {:?}, expr: {:?} }}",
            self.source_position, self.declaration_type, self.ident, self.expr
        )
    }
}

impl Checking for GlobalVarDecl {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting GlobalVarDecl node.");
        // Implement visitGlobalVarDecl function...
    }
}

impl PrintAST for GlobalVarDecl {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.declaration_type.visit_for_printing(depth + 1);
        self.ident.visit_for_printing(depth + 1);
        self.expr.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for GlobalVarDecl {
    fn unparse_to_code(&self, depth: i32) {
        print_indent(depth);
        self.declaration_type.unparse_to_code(depth);
        print!(" ");
        self.ident.unparse_to_code(depth);
        if matches!(*self.declaration_type, AstTypeVariant::Array(_)) {
            print!("[");
            self.expr.unparse_to_code(depth);
            print!("]");
        } else {
            print!(" = ");
            self.expr.unparse_to_code(depth);
        }
        print!(";");
    }
}

impl GlobalVarDecl {
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

#[derive(Clone, Debug, PartialEq)]
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

impl Checking for LocalVarDecl {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting LocalVarDecl node.");
        // Implement visitLocalVarDecl function...
    }
}

impl PrintAST for LocalVarDecl {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.declaration_type.visit_for_printing(depth + 1);
        self.ident.visit_for_printing(depth + 1);
        self.expr.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for LocalVarDecl {
    fn unparse_to_code(&self, depth: i32) {
        print_newline_and_indent(depth);
        self.declaration_type.unparse_to_code(depth);
        print!(" ");
        self.ident.unparse_to_code(depth);
        if let AstTypeVariant::Array(curr_array) = &*self.declaration_type {
            let expr: &ExprType = &curr_array.expression;
            print!("[");
            if !matches!(expr, ExprType::EmptyExpr(_)) {
                curr_array.unparse_to_code(depth);
            }
            print!("]");
        }

        if !matches!(*self.expr, ExprType::EmptyExpr(_)) {
            print!(" = ");
            self.expr.unparse_to_code(depth);
        }
        print!(";");
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

#[derive(Clone, Debug, PartialEq)]
pub struct ParaDecl {
    source_position: SourcePosition,
    declaration_type: Box<AstTypeVariant>,
    ident: Box<Ident>,
}

impl Checking for ParaDecl {
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
            self.source_position, self.declaration_type, self.ident
        )
    }
}

impl PrintAST for ParaDecl {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.declaration_type.visit_for_printing(depth + 1);
        self.ident.visit_for_printing(depth + 1);
    }
}

impl PrintUnparsedAST for ParaDecl {
    fn unparse_to_code(&self, depth: i32) {
        self.declaration_type.unparse_to_code(depth);
        print!(" ");
        self.ident.unparse_to_code(depth);
        if let AstTypeVariant::Array(curr_array) = &*self.declaration_type {
            let expr: &ExprType = &curr_array.expression;
            print!("[");
            if !matches!(expr, ExprType::EmptyExpr(_)) {
                curr_array.unparse_to_code(depth);
            }
            print!("]");
        }
    }
}

impl ParaDecl {
    pub fn new(
        source_position: SourcePosition,
        decl_type: Box<AstTypeVariant>,
        ident: Box<Ident>,
    ) -> Self {
        Self {
            source_position,
            declaration_type: decl_type,
            ident,
        }
    }
}
