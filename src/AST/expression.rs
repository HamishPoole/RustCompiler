use std::fmt;

use crate::ast::Ast;
use crate::ast::ident::Ident;
use crate::ast::list::{ArgList, ListType};
use crate::ast::literals::{BooleanLiteral, FloatLiteral, IntLiteral, Operator, StringLiteral};
use crate::ast::variable::Var;
use crate::utils::SourcePosition;

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

#[derive(Debug)]
pub struct ArrayExpr {
    source_position: SourcePosition,
    v: Var,
    e: Box<ExprType>,
}

impl Ast for ArrayExpr {
    fn visit(&self) {
        println!("Visiting ArrayExpr node.");
        todo!("Implement visitArrayExpr function in checker.rs")
    }
}

impl std::fmt::Display for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, v: {:?}, e: {:?} }}",
            self.source_position, self.v, self.e
        )
    }
}

impl ArrayExpr {
    pub fn new(source_position: SourcePosition, v: Var, e: Box<ExprType>) -> Self {
        Self {
            source_position,
            v,
            e,
        }
    }
}

#[derive(Debug)]
pub struct AssignExpr {
    source_position: SourcePosition,
    e1: Box<ExprType>,
    e2: Box<ExprType>,
}

impl Ast for AssignExpr {
    fn visit(&self) {
        println!("Visiting AssignExpr node.");
        todo!("Implement visitAssignExpr function in checker.rs")
    }
}

impl std::fmt::Display for AssignExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, e1: {:?}, e2: {:?} }}",
            self.source_position, self.e1, self.e2
        )
    }
}

impl AssignExpr {
    pub fn new(source_position: SourcePosition, e1: Box<ExprType>, e2: Box<ExprType>) -> Self {
        Self {
            source_position,
            e1,
            e2,
        }
    }
}

#[derive(Debug)]
pub struct ArrayInitExpr {
    source_position: SourcePosition,
    init_list: Box<ListType>, // Needs definition
}

impl Ast for ArrayInitExpr {
    fn visit(&self) {
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

impl ArrayInitExpr {
    pub fn new(source_position: SourcePosition, il: Box<ListType>) -> Self {
        Self {
            source_position,
            init_list: il,
        }
    }
}

#[derive(Debug)]
pub struct Arg {
    pub(crate) source_position: SourcePosition,
    e: Box<ExprType>,
}

impl Ast for Arg {
    fn visit(&self) {
        println!("Visiting Arg node.");
        todo!("Implement visitArg function in checker.rs")
    }
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, e: {:?} }}",
            self.source_position, self.e
        )
    }
}

impl Arg {
    pub fn new(source_position: SourcePosition, e: Box<ExprType>) -> Self {
        Self { source_position, e }
    }
}

#[derive(Debug)]
pub struct BooleanExpr {
    source_position: SourcePosition,
    boolean_literal: Box<BooleanLiteral>, // Assuming FloatLiteral struct is defined
}

impl Ast for BooleanExpr {
    fn visit(&self) {
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

impl BooleanExpr {
    pub fn new(source_position: SourcePosition, int_literal: Box<BooleanLiteral>) -> Self {
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
    argument_list: Box<ArgList>,
}

impl Ast for CallExpr {
    fn visit(&self) {
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

impl CallExpr {
    pub fn new(source_position: SourcePosition, ident: Ident, argument_list: Box<ArgList>) -> Self {
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

impl fmt::Display for EmptyExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ source_position: {:?} }}", self.source_position)
    }
}

impl EmptyExpr {
    pub fn new(source_position: SourcePosition) -> Self {
        Self { source_position }
    }

    pub fn visit(&self) {
        println!("Visiting EmptyExpr node.");
        todo!("Implement visitEmptyExpr function in checker.rs")
    }
}


#[derive(Debug)]
pub struct VarExpr {
    source_position: SourcePosition,
    v: Box<Var>, // Needs definition
}

impl Ast for VarExpr {
    fn visit(&self) {
        println!("Visiting VarExpr node.");
        todo!("Implement visitVarExpr function in checker.rs")
    }
}

impl fmt::Display for VarExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, v: {:?} }}",
            self.source_position, self.v
        )
    }
}

impl VarExpr {
    pub fn new(source_position: SourcePosition, v: Box<Var>) -> Self {
        Self { source_position, v }
    }
}

#[derive(Debug)]
pub struct FloatExpr {
    source_position: SourcePosition,
    float_literal: Box<FloatLiteral>, // Assuming FloatLiteral struct is defined
}

impl Ast for FloatExpr {
    fn visit(&self) {
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

impl FloatExpr {
    pub fn new(source_position: SourcePosition, fl: Box<FloatLiteral>) -> Self {
        Self {
            source_position,
            float_literal: fl,
        }
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    source_position: SourcePosition,
    e1: Box<ExprType>,
    o: Operator,
    // Assuming Operator struct is defined
    e2: Box<ExprType>,
}

impl Ast for BinaryExpr {
    fn visit(&self) {
        println!("Visiting BinaryExpr node.");
        todo!("Implement visitBinaryExpr function in checker.rs")
    }
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, e1: {:?}, o: {:?}, e2: {:?} }}",
            self.source_position, self.e1, self.o, self.e2
        )
    }
}

impl BinaryExpr {
    pub fn new(
        source_position: SourcePosition,
        e1: Box<ExprType>,
        o: Operator,
        e2: Box<ExprType>,
    ) -> Self {
        Self {
            source_position,
            e1,
            o,
            e2,
        }
    }
}

#[derive(Debug)]
pub struct IntExpr {
    source_position: SourcePosition,
    int_literal: Box<IntLiteral>, // Assuming FloatLiteral struct is defined
}

impl Ast for IntExpr {
    fn visit(&self) {
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

impl IntExpr {
    pub fn new(source_position: SourcePosition, int_literal: Box<IntLiteral>) -> Self {
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
    fn visit(&self) {
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
    o: Operator,
    e: Box<ExprType>,
}

impl Ast for UnaryExpr {
    fn visit(&self) {
        println!("Visiting UnaryExpr node.");
        todo!("Implement visitUnaryExpr function in checker.rs")
    }
}

impl std::fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ source_position: {:?}, o: {:?}, e: {:?} }}",
            self.source_position, self.o, self.e
        )
    }
}

impl UnaryExpr {
    pub fn new(source_position: SourcePosition, o: Operator, e: Box<ExprType>) -> Self {
        Self {
            source_position,
            o,
            e,
        }
    }
}
