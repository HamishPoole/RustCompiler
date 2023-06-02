use std::fmt;

use crate::ast::{Ast, PrintingVisit};
use crate::ast::ident::Ident;
use crate::ast::list::ListType;
use crate::ast::literals::{BooleanLiteral, FloatLiteral, IntLiteral, Operator, StringLiteral};
use crate::ast::variable::VarUntyped;
use crate::globals::TAB_SIZE;
use crate::utils::{generate_tabbed_string, SourcePosition};

#[derive(Debug)]
pub enum ExprType {
    Arg(Arg),
    ArrayExpr(ArrayExpr),
    ArrayInitExpr(ArrayInitExpr),
    AssignExpr(AssignExpr),
    BinaryExpr(BinaryExpr),
    BooleanExpr(BooleanExpr),
    CallExpr(CallExpr),
    EmptyArrayExprList(crate::ast::list::EmptyArrayExprList),
    EmptyExpr(EmptyExpr),
    FloatExpr(FloatExpr),
    IntExpr(IntExpr),
    StringExpr(StringExpr),
    UnaryExpr(UnaryExpr),
    VarExpr(VarExpr),
}

impl ExprType {
    // https://docs.rs/enum_dispatch/latest/enum_dispatch/
    // See this crate for macros to refactor this.
    pub fn get_source_position(&self) -> &SourcePosition {
        match self {
            ExprType::Arg(expr) => &expr.source_position,
            ExprType::ArrayExpr(expr) => &expr.source_position,
            ExprType::ArrayInitExpr(expr) => &expr.source_position,
            ExprType::AssignExpr(expr) => &expr.source_position,
            ExprType::BinaryExpr(expr) => &expr.source_position,
            ExprType::BooleanExpr(expr) => &expr.source_position,
            ExprType::CallExpr(expr) => &expr.source_position,
            ExprType::EmptyArrayExprList(expr) => &expr.source_position,
            ExprType::FloatExpr(expr) => &expr.source_position,
            ExprType::IntExpr(expr) => &expr.source_position,
            ExprType::StringExpr(expr) => &expr.source_position,
            ExprType::UnaryExpr(expr) => &expr.source_position,
            ExprType::VarExpr(expr) => &expr.source_position,
            _ => panic!("Variant does not have a source_position"),
        }
    }
}

impl PrintingVisit for ExprType {
    fn visit_for_printing(&self, depth: i32) {
        match self {
            ExprType::Arg(expr) => expr.visit_for_printing(depth),
            ExprType::ArrayExpr(expr) => expr.visit_for_printing(depth),
            ExprType::ArrayInitExpr(expr) => expr.visit_for_printing(depth),
            ExprType::AssignExpr(expr) => expr.visit_for_printing(depth),
            ExprType::BinaryExpr(expr) => expr.visit_for_printing(depth),
            ExprType::BooleanExpr(expr) => expr.visit_for_printing(depth),
            ExprType::CallExpr(expr) => expr.visit_for_printing(depth),
            ExprType::EmptyArrayExprList(expr) => expr.visit_for_printing(depth),
            ExprType::EmptyExpr(expr) => expr.visit_for_printing(depth),
            ExprType::FloatExpr(expr) => expr.visit_for_printing(depth),
            ExprType::IntExpr(expr) => expr.visit_for_printing(depth),
            ExprType::StringExpr(expr) => expr.visit_for_printing(depth),
            ExprType::UnaryExpr(expr) => expr.visit_for_printing(depth),
            ExprType::VarExpr(expr) => expr.visit_for_printing(depth),
        }
    }
}


#[derive(Debug)]
pub struct Arg {
    pub source_position: SourcePosition,
    expr: Box<ExprType>,
}

impl Ast for Arg {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting Arg node.");
        todo!("Implement visitArg function in checker.rs")
    }
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, e: {:?} }}",
            self.source_position, self.expr
        )
    }
}

impl PrintingVisit for Arg {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expr.visit_for_printing(depth + 1);
    }
}

impl Arg {
    pub fn new(source_position: SourcePosition, e: Box<ExprType>) -> Self {
        Self { source_position, expr: e }
    }
}

#[derive(Debug)]
pub struct ArrayExpr {
    source_position: SourcePosition,
    var: VarUntyped,
    expr: Box<ExprType>,
}

impl Ast for ArrayExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ArrayExpr node.");
        todo!("Implement visitArrayExpr function in checker.rs")
    }
}

impl std::fmt::Display for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, v: {:?}, e: {:?} }}",
            self.source_position, self.var, self.expr
        )
    }
}

impl PrintingVisit for ArrayExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.var.visit_for_printing(depth + 1);
        self.expr.visit_for_printing(depth + 1);
    }
}

impl ArrayExpr {
    pub fn new(source_position: SourcePosition, var: VarUntyped, expr: Box<ExprType>) -> Self {
        Self {
            source_position,
            var,
            expr,
        }
    }
}

#[derive(Debug)]
pub struct AssignExpr {
    source_position: SourcePosition,
    expression_one: Box<ExprType>,
    expression_two: Box<ExprType>,
}

impl Ast for AssignExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting AssignExpr node.");
        todo!("Implement visitAssignExpr function in checker.rs")
    }
}

impl std::fmt::Display for AssignExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, e1: {:?}, e2: {:?} }}",
            self.source_position, self.expression_one, self.expression_two
        )
    }
}

impl PrintingVisit for AssignExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expression_one.visit_for_printing(depth + 1);
        self.expression_two.visit_for_printing(depth + 1);
    }
}

impl AssignExpr {
    pub fn new(source_position: SourcePosition, e1: Box<ExprType>, e2: Box<ExprType>) -> Self {
        Self {
            source_position,
            expression_one: e1,
            expression_two: e2,
        }
    }
}

#[derive(Debug)]
pub struct ArrayInitExpr {
    source_position: SourcePosition,
    init_list: Box<ListType>, // Needs definition
}

impl Ast for ArrayInitExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting ArrayInitExpr node.");
        todo!("Implement visitArrayInitExpr function in checker.rs")
    }
}

impl fmt::Display for ArrayInitExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, il: {:?} }}",
            self.source_position, self.init_list
        )
    }
}

impl PrintingVisit for ArrayInitExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.init_list.visit_for_printing(depth + 1);
    }
}

impl ArrayInitExpr {
    pub fn new(source_position: SourcePosition, init_list: Box<ListType>) -> Self {
        Self {
            source_position,
            init_list,
        }
    }
}


#[derive(Debug)]
pub struct BinaryExpr {
    source_position: SourcePosition,
    expression_one: Box<ExprType>,
    operator: Operator,
    expression_two: Box<ExprType>,
}

impl Ast for BinaryExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting BinaryExpr node.");
        todo!("Implement visitBinaryExpr function in checker.rs")
    }
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, e1: {:?}, o: {:?}, e2: {:?} }}",
            self.source_position, self.expression_one, self.operator, self.expression_two
        )
    }
}

impl PrintingVisit for BinaryExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.expression_one.visit_for_printing(depth + 1);
        self.operator.visit_for_printing(depth + 1);
        self.expression_two.visit_for_printing(depth + 1);
    }
}

impl BinaryExpr {
    pub fn new(
        source_position: SourcePosition,
        expression_one: Box<ExprType>,
        operator: Operator,
        expression_two: Box<ExprType>,
    ) -> Self {
        Self {
            source_position,
            expression_one,
            operator,
            expression_two,
        }
    }
}

#[derive(Debug)]
pub struct BooleanExpr {
    source_position: SourcePosition,
    boolean_literal: BooleanLiteral, // Assuming FloatLiteral struct is defined
}

impl Ast for BooleanExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting BooleanExpr node.");
        todo!("Implement visitBooleanExpr function in checker.rs")
    }
}

impl fmt::Display for BooleanExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, fl: {:?} }}",
            self.source_position, self.boolean_literal
        )
    }
}

impl PrintingVisit for BooleanExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.boolean_literal.visit_for_printing(depth + 1);
    }
}

impl BooleanExpr {
    pub fn new(source_position: SourcePosition, int_literal: BooleanLiteral) -> Self {
        Self {
            source_position,
            boolean_literal: int_literal,
        }
    }
}

#[derive(Debug)]
pub struct CallExpr {
    source_position: SourcePosition,
    ident: Ident,
    argument_list: Box<ListType>,
}

impl Ast for CallExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting CallExpr node.");
        todo!("Implement visitCallExpr function in checker.rs")
    }
}

impl std::fmt::Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, ident: {:?}, argument_list: {:?} }}",
            self.source_position, self.ident, self.argument_list
        )
    }
}


impl PrintingVisit for CallExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.ident.visit_for_printing(depth + 1);
        self.argument_list.visit_for_printing(depth + 1);
    }
}

impl CallExpr {
    pub fn new(source_position: SourcePosition, ident: Ident, argument_list: Box<ListType>) -> Self {
        Self {
            source_position,
            ident,
            argument_list,
        }
    }
}

#[derive(Debug)]
pub struct EmptyExpr {
    source_position: SourcePosition,
}

impl Ast for EmptyExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting EmptyExpr node.");
        todo!("Implement visitEmptyExpr function in checker.rs")
    }
}

impl fmt::Display for EmptyExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ source_position: {:?} }}", self.source_position)
    }
}

impl PrintingVisit for EmptyExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
    }
}

impl EmptyExpr {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }
}


#[derive(Debug)]
pub struct FloatExpr {
    source_position: SourcePosition,
    float_literal: FloatLiteral, // Assuming FloatLiteral struct is defined
}

impl Ast for FloatExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting FloatExpr node.");
        todo!("Implement visitFloatExpr function in checker.rs")
    }
}

impl fmt::Display for FloatExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, fl: {:?} }}",
            self.source_position, self.float_literal
        )
    }
}

impl PrintingVisit for FloatExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.float_literal.visit_for_printing(depth + 1);
    }
}

impl FloatExpr {
    pub fn new(source_position: SourcePosition, float_literal: FloatLiteral) -> Self {
        Self {
            source_position,
            float_literal,
        }
    }
}


#[derive(Debug)]
pub struct IntExpr {
    source_position: SourcePosition,
    int_literal: IntLiteral, // Assuming FloatLiteral struct is defined
}

impl Ast for IntExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting IntExpr node.");
        todo!("Implement visitIntExpr function in checker.rs")
    }
}

impl fmt::Display for IntExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, fl: {:?} }}",
            self.source_position, self.int_literal
        )
    }
}

impl PrintingVisit for IntExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.int_literal.visit_for_printing(depth + 1);
    }
}

impl IntExpr {
    pub fn new(source_position: SourcePosition, int_literal: IntLiteral) -> Self {
        Self {
            source_position,
            int_literal,
        }
    }
}

#[derive(Debug)]
pub struct StringExpr {
    source_position: SourcePosition,
    string_literal: Box<StringLiteral>, // Assuming FloatLiteral struct is defined
}

impl Ast for StringExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting StringExpr node.");
        todo!("Implement visitStringExpr function in checker.rs")
    }
}

impl fmt::Display for StringExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, fl: {:?} }}",
            self.source_position, self.string_literal
        )
    }
}

impl PrintingVisit for StringExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.string_literal.visit_for_printing(depth + 1);
    }
}

impl StringExpr {
    pub fn new(source_position: SourcePosition, string_literal: Box<StringLiteral>) -> Self {
        Self {
            source_position,
            string_literal,
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub(crate) source_position: SourcePosition,
    operator: Operator,
    expression: Box<ExprType>,
}

impl Ast for UnaryExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting UnaryExpr node.");
        todo!("Implement visitUnaryExpr function in checker.rs")
    }
}

impl std::fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, o: {:?}, e: {:?} }}",
            self.source_position, self.operator, self.expression
        )
    }
}

impl PrintingVisit for UnaryExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.operator.visit_for_printing(depth + 1);
        self.expression.visit_for_printing(depth + 1);
    }
}

impl UnaryExpr {
    pub fn new(source_position: SourcePosition, operator: Operator, expr: Box<ExprType>) -> Self {
        Self {
            source_position,
            operator,
            expression: expr,
        }
    }
}

#[derive(Debug)]
pub struct VarExpr {
    source_position: SourcePosition,
    var: VarUntyped,
}

impl Ast for VarExpr {
    fn visit_for_semantics_checking(&self) {
        println!("Visiting VarExpr node.");
        todo!("Implement visitVarExpr function in checker.rs")
    }
}

impl fmt::Display for VarExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, v: {:?} }}",
            self.source_position, self.var
        )
    }
}

impl PrintingVisit for VarExpr {
    fn visit_for_printing(&self, depth: i32) {
        let tabbed_string = generate_tabbed_string(
            std::any::type_name::<Self>(), depth);
        println!("{}", tabbed_string);
        self.var.visit_for_printing(depth + 1);
    }
}

impl VarExpr {
    pub fn new(source_position: SourcePosition, var: VarUntyped) -> Self {
        Self { source_position, var }
    }
}

