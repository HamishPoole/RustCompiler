// Can visualise using https://crates.io/crates/graphviz-rust https://docs.rs/petgraph/latest/petgraph/ if needed.
// Arc<Mutex<T>> or RC or RefCell for references.

use std::collections::HashSet;
use std::sync::Arc;

use log::{debug, error};
use once_cell::sync::Lazy;

use crate::ast::array_type::{ArrayType, AstTypeVariant};
use crate::ast::decl::{DeclType, FuncDecl, GlobalVarDecl, LocalVarDecl, ParaDecl};
use crate::ast::expression::{
    Arg, ArrayExpr, ArrayInitExpr, AssignExpr, BinaryExpr, BooleanExpr, CallExpr, EmptyExpr,
    ExprType, FloatExpr, IntExpr, StringExpr, UnaryExpr, VarExpr,
};
use crate::ast::ident::Ident;
use crate::ast::list::{
    ArgList, ArrayExprList, DeclList, EmptyArgList, EmptyArrayExprList, EmptyParamList, ListType,
    ParamList,
};
use crate::ast::list::{EmptyDeclList, EmptyStmtList, StmtList};
use crate::ast::literals::{BooleanLiteral, FloatLiteral, IntLiteral, Operator, StringLiteral};
use crate::ast::primitive_types::{
    AstTypes, BooleanType, FloatType, IntType, StringType, VoidType,
};
use crate::ast::program::Program;
use crate::ast::statement::{
    BreakStmt, CompoundStmt, ContinueStmt, EmptyStmt, ExprStmt, ForStmt, IfStmt, ReturnStmt,
    StmtType, WhileStmt,
};
use crate::ast::variable::VarUntyped;
use crate::ast::AstNode;
use crate::scanner::Scanner;
use crate::token::{Token, TokenKind};
use crate::utils::SourcePosition;

// Option types excellent for handling recoverable errors.
// For errors you shouldn't recover from, just panic!.
pub enum ParserResult {
    Ok(AstNode),
    NoMatch { expect: &'static str },
    Error { error_string: &'static str },
}

#[derive(Debug)]
pub struct ParserData {
    scanner: Scanner,
    current_token: Token,
    current_position: SourcePosition,
}

// NB.  No errors should be recoverable in this parser.
// Recoverable errors imply the parser should handle it.  Syntax errors are not recoverable.
impl ParserData {
    pub fn new(mut scanner: Scanner) -> Self {
        let current_token = scanner.get_next_token();

        Self {
            scanner,
            current_token,
            current_position: SourcePosition::new(0, 0, 0, 0),
        }
    }
}

pub fn parse_code(scanner: Scanner) -> Program {
    let mut parser_data = ParserData::new(scanner);

    parse_program(&mut parser_data)
}

pub fn parse_program(parser_struct: &mut ParserData) -> Program {
    let declaration_list = parse_declaration_list(parser_struct, true);

    match declaration_list {
        ListType::DeclList(decl_list) => Program::new(decl_list),
        _ => panic!("Error: Expected declaration list."),
    }
}

fn parse_declaration_list(parser_struct: &mut ParserData, is_global: bool) -> ListType {
    let start = parser_struct.current_position;

    if token_is_primitive_type(&parser_struct.current_token.token_kind) {
        let curr_type = parse_type(parser_struct);
        let ident = parse_identifier(parser_struct);
        if parser_struct.current_token.token_kind == TokenKind::LPAREN {
            parse_func_decl_list(Box::new(curr_type), ident, is_global, parser_struct)
        } else {
            parse_initial_declaration_list(
                parser_struct,
                Box::new(curr_type),
                Box::new(ident),
                is_global,
            )
        }
    } else {
        ListType::EmptyDeclList(EmptyDeclList::new(start))
    }
}

// func-decl -> identifier para-list compound-stmt
// var-decl -> init-declarator-list ";"
fn parse_func_decl_list(
    function_type: Box<AstTypes>,
    ident: Ident,
    is_global: bool,
    parser_struct: &mut ParserData,
) -> ListType {
    let start_pos = parser_struct.current_position;

    let lhs_child = Box::new(DeclType::FuncDecl(parse_func_decl(
        function_type.clone(),
        ident.copy_with_null_decl(),
        parser_struct,
    )));

    let rhs_child = if parser_struct.current_token.token_kind != TokenKind::EOF {
        Box::new(parse_declaration_list(parser_struct, is_global))
    } else {
        Box::new(ListType::EmptyDeclList(EmptyDeclList::new(
            parser_struct.current_position,
        )))
    };

    let final_pos = finish_position(&start_pos, &parser_struct.current_position);
    ListType::DeclList(DeclList::new(final_pos, lhs_child, rhs_child))
}

// func-decl -> identifier para-list compound-stmt
fn parse_func_decl(
    function_type: Box<AstTypes>,
    ident: Ident,
    parser_struct: &mut ParserData,
) -> FuncDecl {
    let start_pos = parser_struct.current_position;

    let function_parameter_list_ast = parse_parameter_list(parser_struct);
    let compound_stmt_ast = parse_compound_stmt(parser_struct);

    let final_pos = finish_position(&start_pos, &parser_struct.current_position);

    FuncDecl::new(
        final_pos,
        Box::new(AstTypeVariant::Primitive(*function_type)),
        Box::new(ident),
        Box::new(function_parameter_list_ast),
        Box::new(compound_stmt_ast),
    )
}

// init-declarator-list-> init-declarator ( "," init-declarator )*
fn parse_initial_declaration_list(
    parser_struct: &mut ParserData,
    decl_type: Box<AstTypes>,
    identifier: Box<Ident>,
    is_global: bool,
) -> ListType {
    let start_pos = parser_struct.current_position;

    let ident_spelling = (*identifier).spelling.clone();
    let ident_source_pos = (*identifier).source_position;
    let ident_clone = Box::new(Ident::new(ident_spelling, ident_source_pos, None));

    let lhs_child =
        parse_initial_declarator(parser_struct, decl_type.clone(), ident_clone, is_global);
    let mut rhs_child = ListType::EmptyDeclList(EmptyDeclList::new(parser_struct.current_position));

    if parser_struct.current_token.token_kind == TokenKind::COMMA {
        // Array initialisation expression.
        match_and_consume_next_token(parser_struct); // COMMA

        let rhs_child =
            parse_initial_declaration_list(parser_struct, decl_type, identifier, is_global);
        let final_pos = finish_position(&start_pos, &parser_struct.current_position);

        return ListType::DeclList(DeclList::new(
            final_pos,
            Box::new(lhs_child),
            Box::new(rhs_child),
        ));
    }

    match_and_consume_next_token(parser_struct); // SEMICOLON.

    if token_is_primitive_type(&parser_struct.current_token.token_kind) {
        rhs_child = parse_declaration_list(parser_struct, is_global);
    }

    let final_pos = finish_position(&start_pos, &parser_struct.current_position);

    ListType::DeclList(DeclList::new(
        final_pos,
        Box::new(lhs_child),
        Box::new(rhs_child),
    ))
}

// init-declarator -> declarator ( "=" initializer )?
fn parse_initial_declarator(
    parser_struct: &mut ParserData,
    decl_type: Box<AstTypes>,
    identifier: Box<Ident>,
    is_global: bool,
) -> DeclType {
    let start_pos = parser_struct.current_position;
    let var_type = parse_declarator(parser_struct, decl_type);
    let mut init_expr = ExprType::EmptyExpr(EmptyExpr::new(parser_struct.current_position));

    if parser_struct.current_token.token_kind == TokenKind::EQ {
        match_and_consume_next_token(parser_struct); // EQ
        init_expr = parse_initialiser(parser_struct);
    }

    let final_source_pos = finish_position(&start_pos, &parser_struct.current_position);

    if is_global {
        DeclType::GlobalVarDecl(GlobalVarDecl::new(
            final_source_pos,
            Box::new(var_type),
            identifier,
            Box::new(init_expr),
        ))
    } else {
        DeclType::LocalVarDecl(LocalVarDecl::new(
            final_source_pos,
            Box::new(var_type),
            identifier,
            Box::new(init_expr),
        ))
    }
}

// declarator -> identifier
// | identifier "[" INTLITERAL? "]"
fn parse_declarator(parser_struct: &mut ParserData, decl_type: Box<AstTypes>) -> AstTypeVariant {
    let start_pos = parser_struct.current_position;

    match parser_struct.current_token.token_kind {
        TokenKind::LBRACKET => {
            match_and_consume_next_token(parser_struct); // LBRACKET
            let expression = parse_array_type_expr(parser_struct);
            match_and_consume_next_token(parser_struct); // RBRACKET
            let final_position =
                finish_position(&start_pos, &parser_struct.current_token.token_position);

            AstTypeVariant::Array(ArrayType::new(
                final_position,
                Arc::new(decl_type),
                expression,
            ))
        }
        _ => AstTypeVariant::Primitive(*decl_type),
    }
}

fn parse_array_type_expr(parser_struct: &mut ParserData) -> ExprType {
    let start_pos = parser_struct.current_position;

    match parser_struct.current_token.token_kind {
        TokenKind::INTLITERAL => parse_expr(parser_struct),
        _ => ExprType::EmptyExpr(EmptyExpr::new(start_pos)),
    }
}

// initialiser -> expr
// | "{" expr ( "," expr )* "}"
fn parse_initialiser(parser_struct: &mut ParserData) -> ExprType {
    let start_pos = parser_struct.current_position;

    match parser_struct.current_token.token_kind {
        TokenKind::LBRACE => {
            match_and_consume_next_token(parser_struct); // Consume LBRACE.
            let array_expr_list = parse_array_expr_list(parser_struct);
            let finish_pos = finish_position(&start_pos, &parser_struct.current_position);
            ExprType::ArrayInitExpr(ArrayInitExpr::new(finish_pos, Box::new(array_expr_list)))
        }
        _ => parse_expr(parser_struct),
    }
}

fn parse_array_expr_list(parser_struct: &mut ParserData) -> ListType {
    let start_pos = parser_struct.current_position;

    let lhs_expr = parse_expr(parser_struct);

    match parser_struct.current_token.token_kind {
        TokenKind::COMMA => {
            match_and_consume_next_token(parser_struct); // Consume COMMA.
            let rhs_array_expr = parse_array_expr_list(parser_struct);
            let finish_pos = finish_position(&start_pos, &parser_struct.current_position);

            ListType::ArrayExprList(ArrayExprList::new(
                finish_pos,
                lhs_expr,
                Box::new(rhs_array_expr),
            ))
        }
        _ => {
            let finish_pos = finish_position(&start_pos, &parser_struct.current_position);
            match_and_consume_next_token(parser_struct); // Consume RBRACE.
            ListType::ArrayExprList(ArrayExprList::new(
                finish_pos,
                lhs_expr,
                Box::new(ListType::EmptyArrayExprList(EmptyArrayExprList::new(
                    finish_pos,
                ))),
            ))
        }
    }
}

// ======================================== Statements =============================================

// compound-stmt -> "{" var-decl* stmt* "}"
fn parse_compound_stmt(parser_struct: &mut ParserData) -> StmtType {
    let start_pos = parser_struct.current_position;
    match_and_consume_next_token(parser_struct); // Consumes LBRACE.
    let mut declare_list = ListType::EmptyDeclList(EmptyDeclList::new(start_pos));

    if token_is_primitive_type(&parser_struct.current_token.token_kind) {
        declare_list = parse_declaration_list(parser_struct, false);
    }

    let stmt_list_ast = parse_stmt_list(parser_struct);
    // Consumers RCURLY
    match_and_consume_next_token(parser_struct);
    let final_source_pos = finish_position(&start_pos, &parser_struct.current_position);

    StmtType::CompoundStmt(CompoundStmt::new(
        Box::new(declare_list),
        Box::new(stmt_list_ast),
        final_source_pos,
    ))
}

// Note, need to implement multiple statements being recursively parsed.
fn parse_stmt_list(parser_struct: &mut ParserData) -> ListType {
    let start_pos = parser_struct.current_position;

    if parser_struct.current_token.token_kind == TokenKind::RBRACE {
        return ListType::EmptyStmtList(EmptyStmtList::new(start_pos));
    }

    let lhs_single_stmt = parse_single_stmt(parser_struct);
    let final_source_pos = finish_position(&start_pos, &parser_struct.current_position.clone());

    let rhs = StmtList::new(
        final_source_pos,
        Box::new(lhs_single_stmt),
        Box::new(parse_stmt_list(parser_struct)),
    );

    ListType::StmtList(rhs)
}

// stmt -> compound-stmt
// | if-stmt
// | for-stmt
// | while-stmt
// | break-stmt
// | continue-stmt
// | return-stmt
// | expr-stmt
fn parse_single_stmt(parser_struct: &mut ParserData) -> StmtType {
    match parser_struct.current_token.token_kind {
        TokenKind::LBRACE => parse_compound_stmt(parser_struct),
        TokenKind::IF => parse_if_statement(parser_struct),
        TokenKind::FOR => parse_for_statement(parser_struct),
        TokenKind::WHILE => parse_while_stmt(parser_struct),
        TokenKind::BREAK => parse_break_stmt(parser_struct),
        TokenKind::CONTINUE => parse_continue_stmt(parser_struct),
        TokenKind::RETURN => parse_return_stmt(parser_struct),
        _ => {
            if EXPR_FIRST_SET.contains(&parser_struct.current_token.token_kind) {
                parse_expr_stmt(parser_struct)
            } else {
                panic!(
                    "Invalid token kind for single statement: {:?}, at position: {:?}",
                    parser_struct.current_token.token_kind, parser_struct.current_position
                );
            }
        }
    }
}

fn parse_single_or_multiple_statements(parser_struct: &mut ParserData) -> StmtType {
    if parser_struct.current_token.token_kind == TokenKind::LBRACE {
        parse_compound_stmt(parser_struct)
    } else {
        parse_single_stmt(parser_struct)
    }
}

// if-stmt -> if "(" expr ")" stmt ( else stmt )?
fn parse_if_statement(parser_struct: &mut ParserData) -> StmtType {
    let start_pos = parser_struct.current_position;

    match_and_consume_next_token(parser_struct); // Consume TokenKind::IF
    match_and_consume_next_token(parser_struct); // Consume TokenKind::LPAREN

    let if_expr = parse_expr(parser_struct);
    match_and_consume_next_token(parser_struct); // Consume TokenKind::RPAREN

    let if_stmt = parse_single_or_multiple_statements(parser_struct);

    if parser_struct.current_token.token_kind != TokenKind::ELSE {
        let final_source_pos = finish_position(&start_pos, &parser_struct.current_position);
        return StmtType::IfStmt(IfStmt::new(
            final_source_pos,
            Box::new(if_expr),
            Box::new(if_stmt),
            Box::new(StmtType::EmptyStmt(EmptyStmt::new(final_source_pos))),
        ));
    }

    match_and_consume_next_token(parser_struct); // Consume TokenKind::ELSE
    let else_stmt = parse_single_or_multiple_statements(parser_struct);
    let final_source_pos = finish_position(&start_pos, &parser_struct.current_position);
    StmtType::IfStmt(IfStmt::new(
        final_source_pos,
        Box::new(if_expr),
        Box::new(if_stmt),
        Box::new(else_stmt),
    ))
}

// for-stmt -> for "(" expr? ";" expr? ";" expr? ")" stmt
fn parse_for_statement(parser_struct: &mut ParserData) -> StmtType {
    let start_pos = parser_struct.current_position;

    match_and_consume_next_token(parser_struct); // Consume TokenKind::FOR
    match_and_consume_next_token(parser_struct); // Consume TokenKind::LPAREN

    let expr1 = parse_expr(parser_struct);
    match_and_consume_next_token(parser_struct); // Consume TokenKind::SEMICOLON

    let expr2 = parse_expr(parser_struct);
    match_and_consume_next_token(parser_struct); // Consume TokenKind::SEMICOLON

    let mut expr3 = ExprType::EmptyExpr(EmptyExpr::new(parser_struct.current_position));
    if EXPR_FIRST_SET.contains(&parser_struct.current_token.token_kind) {
        expr3 = parse_expr(parser_struct);
    }
    match_and_consume_next_token(parser_struct); // Consume TokenKind::RPAREN

    let final_source_pos = finish_position(&start_pos, &parser_struct.current_position);

    StmtType::ForStmt(ForStmt::new(
        final_source_pos,
        Box::new(expr1),
        Box::new(expr2),
        Box::new(expr3),
        Box::new(parse_single_or_multiple_statements(parser_struct)),
    ))
}

// while-stmt -> while "(" expr ")" stmt
fn parse_while_stmt(parser_struct: &mut ParserData) -> StmtType {
    let start_pos = parser_struct.current_token.token_position;

    match_and_consume_next_token(parser_struct); // Consume TokenKind::WHILE
    match_and_consume_next_token(parser_struct); // Consume TokenKind::LPAREN

    let while_condition_ast = Box::new(parse_expr(parser_struct));

    match_and_consume_next_token(parser_struct); // Consume TokenKind::RPAREN

    let while_stmt = parse_single_or_multiple_statements(parser_struct);

    let final_position = finish_position(
        &start_pos,
        &parser_struct.current_token.token_position.clone(),
    );

    StmtType::WhileStmt(WhileStmt::new(
        final_position,
        while_condition_ast,
        Box::new(while_stmt),
    ))
}

// break-stmt -> break ";"
fn parse_break_stmt(parser_struct: &mut ParserData) -> StmtType {
    let start_pos = parser_struct.current_token.token_position;

    match_and_consume_next_token(parser_struct); // Consume TokenKind::BREAK
    match_and_consume_next_token(parser_struct); // Consume TokenKind::SEMICOLON

    let final_position = finish_position(
        &start_pos,
        &parser_struct.current_token.token_position.clone(),
    );

    StmtType::BreakStmt(BreakStmt::new(final_position))
}

// continue-stmt       -> continue ";"
fn parse_continue_stmt(parser_struct: &mut ParserData) -> StmtType {
    let start_pos = parser_struct.current_token.token_position;
    match_and_consume_next_token(parser_struct);
    match_and_consume_next_token(parser_struct);

    StmtType::ContinueStmt(ContinueStmt::new(start_pos))
}

// return-stmt         -> return expr? ";"
fn parse_return_stmt(parser_struct: &mut ParserData) -> StmtType {
    let start_pos = parser_struct.current_token.token_position;

    match_and_consume_next_token(parser_struct); // Consume TokenKind::RETURN

    if EXPR_FIRST_SET.contains(&parser_struct.current_token.token_kind) {
        let expr = Box::new(parse_expr(parser_struct));
        let final_position = finish_position(
            &start_pos,
            &parser_struct.current_token.token_position.clone(),
        );
        return StmtType::ReturnStmt(ReturnStmt::new(final_position, expr));
    };

    // Consume TokenKind::SEMICOLON
    match_and_consume_next_token(parser_struct);
    let final_position = finish_position(
        &start_pos,
        &parser_struct.current_token.token_position.clone(),
    );
    let empty_expr = ExprType::EmptyExpr(EmptyExpr::new(final_position));

    StmtType::ReturnStmt(ReturnStmt::new(final_position, Box::new(empty_expr)))
}

// ====================================== Expressions ==============================================

// expr-stmt -> expr? ";"
fn parse_expr_stmt(parser_struct: &mut ParserData) -> StmtType {
    // Doesn't consume )
    let mut start_pos = parser_struct.current_position;

    match parser_struct.current_token.token_kind {
        kind if EXPR_FIRST_SET.contains(&kind) => {
            let expr_ast = parse_expr(parser_struct);
            match_and_consume_next_token(parser_struct); // Consume TokenKind::SEMICOLON
            let final_source_pos = finish_position(&start_pos, &parser_struct.current_position);
            let expr_stmt = ExprStmt::new(final_source_pos, expr_ast);
            StmtType::ExprStmt(expr_stmt)
        }
        TokenKind::SEMICOLON => {
            match_and_consume_next_token(parser_struct);
            let final_source_pos = finish_position(
                &start_pos,
                &parser_struct.current_token.token_position.clone(),
            );

            let empty_expr = ExprType::EmptyExpr(EmptyExpr::new(final_source_pos));
            StmtType::ExprStmt(ExprStmt::new(final_source_pos, empty_expr))
        }
        _ => {
            panic!("Error in expr statement.");
        }
    }
}

// expr -> assignment-expr
fn parse_expr(parser_struct: &mut ParserData) -> ExprType {
    if let TokenKind::SEMICOLON = parser_struct.current_token.token_kind {
        return ExprType::EmptyExpr(EmptyExpr::new(parser_struct.current_position));
    }
    parse_assign_expr(parser_struct)
}

// assignment-expr  -> cond-or-expr assignment-expr
fn parse_assign_expr(parser_struct: &mut ParserData) -> ExprType {
    let mut assign_expr_pos = parser_struct.current_token.token_position;
    let mut lhs_expr = parse_conditional_or_expression(parser_struct);

    match parser_struct.current_token.token_kind {
        TokenKind::EQ => {
            let eq = consume_operator(parser_struct);
            let rhs = parse_assign_expr(parser_struct);
            let assign_expr = AssignExpr::new(
                finish_position(&assign_expr_pos, &rhs.get_source_position()),
                Box::new(lhs_expr),
                Box::new(rhs),
            );
            ExprType::AssignExpr(assign_expr)
        }
        _ => lhs_expr,
    }
}

// cond-or-expr        -> cond-and-expr cond-or-expr'
fn parse_conditional_or_expression(parser_struct: &mut ParserData) -> ExprType {
    let mut cond_or_expr_pos = parser_struct.current_token.token_position.clone();
    let main_expr = parse_conditional_and_expression(parser_struct);
    parse_conditional_or_expression_tail(&cond_or_expr_pos, main_expr, parser_struct)
}

// cond-or-expr'      -> "||" cond-and-expr cond-or-expr'
// |  ε
fn parse_conditional_or_expression_tail(
    start_pos: &SourcePosition,
    first_expr: ExprType,
    parser_struct: &mut ParserData,
) -> ExprType {
    let mut cond_or_expr_tail_pos = start_pos.clone();
    match parser_struct.current_token.token_kind {
        TokenKind::OROR => {
            let or = consume_operator(parser_struct);
            let second_expr = parse_conditional_and_expression(parser_struct);
            let binary_expr = BinaryExpr::new(
                finish_position(start_pos, &second_expr.get_source_position()),
                Box::new(first_expr),
                or,
                Box::new(second_expr),
            );
            parse_conditional_or_expression_tail(
                &cond_or_expr_tail_pos,
                ExprType::BinaryExpr(binary_expr),
                parser_struct,
            )
        }
        _ => first_expr,
    }
}

// *cond-and-expr* -> equality-expr cond-and-expr'
fn parse_conditional_and_expression(parser_struct: &mut ParserData) -> ExprType {
    let mut cond_and_expr_pos = parser_struct.current_token.token_position.clone();
    let mut main_expr = parse_equality_expression(parser_struct);
    parse_conditional_and_expression_tail(&cond_and_expr_pos, main_expr, parser_struct)
}

// cond-and-expr' -> "&&" equality-expr cond-and-expr' | epsilon
fn parse_conditional_and_expression_tail(
    start_pos: &SourcePosition,
    first_expr: ExprType,
    parser_struct: &mut ParserData,
) -> ExprType {
    let mut and_expr_tail_pos = start_pos.clone();

    match parser_struct.current_token.token_kind {
        TokenKind::ANDAND => {
            let and = consume_operator(parser_struct);
            let second_expr = parse_equality_expression(parser_struct);
            let binary_expr = BinaryExpr::new(
                finish_position(start_pos, &second_expr.get_source_position()),
                Box::new(first_expr),
                and,
                Box::new(second_expr),
            );
            parse_conditional_and_expression_tail(
                &and_expr_tail_pos,
                ExprType::BinaryExpr(binary_expr),
                parser_struct,
            )
        }
        _ => first_expr,
    }
}

// *equality-expr* -> rel-expr equality-expr'
fn parse_equality_expression(parser_struct: &mut ParserData) -> ExprType {
    let mut equality_expr_pos = parser_struct.current_token.token_position.clone();
    let mut lhs_expr = parse_relational_expression(parser_struct);
    parse_equality_expression_tail(&equality_expr_pos, lhs_expr, parser_struct)
}

// equality-expr' -> "==" rel-expr equality-expr'
// | "!=" rel-expr equality-expr'
// | epsilon
fn parse_equality_expression_tail(
    start_pos: &SourcePosition,
    lhs_expr: ExprType,
    parser_struct: &mut ParserData,
) -> ExprType {
    let mut equality_expr_tail_pos = parser_struct.current_token.token_position.clone();
    match parser_struct.current_token.token_kind {
        TokenKind::EQEQ | TokenKind::NOTEQ => {
            let op = consume_operator(parser_struct);
            let rhs_expr = parse_relational_expression(parser_struct);
            let binary_expr = BinaryExpr::new(
                finish_position(start_pos, &rhs_expr.get_source_position()),
                Box::new(lhs_expr),
                op,
                Box::new(rhs_expr),
            );
            parse_equality_expression_tail(
                &equality_expr_tail_pos,
                ExprType::BinaryExpr(binary_expr),
                parser_struct,
            )
        }
        _ => lhs_expr,
    }
}

// rel-expr* -> additive-expr rel-expr'
fn parse_relational_expression(parser_struct: &mut ParserData) -> ExprType {
    let mut rel_expr_pos = parser_struct.current_token.token_position.clone();
    let lhs_expr = parse_additive_expression(parser_struct);
    parse_relational_expression_tail(&rel_expr_pos, lhs_expr, parser_struct)
}

// rel-expr' -> "<" additive-expr rel-expr'
// |  "<=" additive-expr rel-expr'
// |  ">" additive-expr rel-expr'
// |  ">=" additive-expr rel-expr'
// |  ε
fn parse_relational_expression_tail(
    start_pos: &SourcePosition,
    lhs_expr: ExprType,
    parser_struct: &mut ParserData,
) -> ExprType {
    let mut pos = parser_struct.current_token.token_position.clone();
    match parser_struct.current_token.token_kind {
        TokenKind::LT | TokenKind::LTEQ | TokenKind::GT | TokenKind::GTEQ => {
            let op = consume_operator(parser_struct);
            let rhs_expr = parse_additive_expression(parser_struct);
            let binary_expr = BinaryExpr::new(
                finish_position(start_pos, &rhs_expr.get_source_position()),
                Box::new(lhs_expr),
                op,
                Box::new(rhs_expr),
            );
            parse_relational_expression_tail(&pos, ExprType::BinaryExpr(binary_expr), parser_struct)
        }
        _ => lhs_expr,
    }
}

// additive-expr       -> multiplicative-expr additive-expr'
fn parse_additive_expression(parser_struct: &mut ParserData) -> ExprType {
    let mut pos = parser_struct.current_token.token_position.clone();
    let lhs_expr = parse_multiplicative_expression(parser_struct);
    parse_additive_expression_tail(&pos, lhs_expr, parser_struct)
}

// additive-expr'      | "+" multiplicative-expr additive-expr'
// |  "-" multiplicative-expr additive-expr'
// |  ε
fn parse_additive_expression_tail(
    start_pos: &SourcePosition,
    lhs_expr: ExprType,
    parser_struct: &mut ParserData,
) -> ExprType {
    let mut pos = parser_struct.current_token.token_position.clone();
    match parser_struct.current_token.token_kind {
        TokenKind::PLUS | TokenKind::MINUS => {
            let op = consume_operator(parser_struct);
            let rhs_expr = parse_multiplicative_expression(parser_struct);
            let binary_expr = BinaryExpr::new(
                finish_position(start_pos, &rhs_expr.get_source_position()),
                Box::new(lhs_expr),
                op,
                Box::new(rhs_expr),
            );
            parse_additive_expression_tail(&pos, ExprType::BinaryExpr(binary_expr), parser_struct)
        }
        _ => lhs_expr,
    }
}

// multiplicative-expr -> unary-expr multiplicative-expr'
fn parse_multiplicative_expression(parser_struct: &mut ParserData) -> ExprType {
    let mut start_pos = parser_struct.current_token.token_position;
    let lhs_expr = parse_unary_expression(parser_struct);
    parse_multiplicative_expression_tail(&start_pos, lhs_expr, parser_struct)
}

// multiplicative-expr'-> "*" unary-expr multiplicative-expr'
// | "/" unary-expr multiplicative-expr'
// |  ε
fn parse_multiplicative_expression_tail(
    start_pos: &SourcePosition,
    lhs_expr: ExprType,
    parser_struct: &mut ParserData,
) -> ExprType {
    let mut pos = parser_struct.current_token.token_position;
    match parser_struct.current_token.token_kind {
        TokenKind::MULT | TokenKind::DIV => {
            let op = consume_operator(parser_struct);
            let rhs_expr = parse_unary_expression(parser_struct);
            let binary_expr = BinaryExpr::new(
                finish_position(start_pos, &rhs_expr.get_source_position()),
                Box::new(lhs_expr),
                op,
                Box::new(rhs_expr),
            );
            ExprType::BinaryExpr(binary_expr)
        }
        _ => lhs_expr,
    }
}

// unary-expr          -> "+" un
// |  "!" unary-expr
// |  primary-expr
fn parse_unary_expression(parser_struct: &mut ParserData) -> ExprType {
    let mut start_pos = parser_struct.current_token.token_position; // assuming `new` is a constructor for `SourcePosition`

    match parser_struct.current_token.token_kind {
        TokenKind::PLUS | TokenKind::MINUS | TokenKind::NOT => {
            let op = consume_operator(parser_struct);
            let lhs_expr = parse_unary_expression(parser_struct);
            let unary = UnaryExpr::new(
                finish_position(&start_pos, &lhs_expr.get_source_position()),
                op,
                Box::new(lhs_expr),
            );
            ExprType::UnaryExpr(unary)
        }

        _ => parse_primary_expr(parser_struct),
    }
}

// primary-expr        -> identifier arg-list?
// | identifier "[" expr "]"
// | "(" expr ")"
// | INTLITERAL
// | FLOATLITERAL
// | BOOLLITERAL
// | STRINGLITERAL
fn parse_primary_expr(parser_struct: &mut ParserData) -> ExprType {
    let start_pos = parser_struct.current_token.token_position;

    match parser_struct.current_token.token_kind {
        TokenKind::ID => {
            let lhs_ident = parse_identifier(parser_struct);
            let sim_vast = VarUntyped::new(
                parser_struct.current_position.clone(),
                lhs_ident.copy_with_null_decl(),
            );

            match parser_struct.current_token.token_kind {
                TokenKind::LBRACKET => {
                    match_and_consume_next_token(parser_struct); // consume '['
                    let rhs_expr = parse_expr(parser_struct);
                    match_and_consume_next_token(parser_struct); // consume ']'

                    let finish_pos = finish_position(&start_pos, &parser_struct.current_position);
                    ExprType::ArrayExpr(ArrayExpr::new(finish_pos, sim_vast, Box::new(rhs_expr)))
                }
                TokenKind::LPAREN => {
                    let args = parse_argument_list(parser_struct);
                    let finish_pos = finish_position(&start_pos, &parser_struct.current_position);

                    ExprType::CallExpr(CallExpr::new(finish_pos, lhs_ident, Box::new(args)))
                }
                _ => {
                    let finish_pos = finish_position(&start_pos, &parser_struct.current_position);
                    ExprType::VarExpr(VarExpr::new(finish_pos, sim_vast))
                }
            }
        }
        TokenKind::LPAREN => {
            match_and_consume_next_token(parser_struct); // consume '('
            let expr_ast = parse_expr(parser_struct);
            match_and_consume_next_token(parser_struct); // consume ')'
            expr_ast
        }
        TokenKind::INTLITERAL => {
            let int_literal = parse_int_literal(parser_struct);
            let final_pos = finish_position(&start_pos, &parser_struct.current_position);
            ExprType::IntExpr(IntExpr::new(final_pos, int_literal))
        }
        TokenKind::FLOATLITERAL => {
            let float_literal = parse_float_literal(parser_struct);
            let final_pos = finish_position(&start_pos, &parser_struct.current_position);
            ExprType::FloatExpr(FloatExpr::new(final_pos, float_literal))
        }
        TokenKind::BOOLEANLITERAL => {
            let bool_literal = parse_boolean_literal(parser_struct);
            let final_pos = finish_position(&start_pos, &parser_struct.current_position);
            ExprType::BooleanExpr(BooleanExpr::new(final_pos, bool_literal))
        }
        TokenKind::STRINGLITERAL => {
            let string_literal = parse_string_literal(parser_struct);
            match string_literal {
                ParserResult::Ok(ast_node) => match ast_node {
                    AstNode::StringLiteral(string_lit) => {
                        let string_expr =
                            StringExpr::new(string_lit.source_position, Box::new(string_lit));
                        ExprType::StringExpr(string_expr)
                    }
                    _ => panic!("Expected a StringLiteral"),
                },
                ParserResult::Error { error_string } => panic!("Should be no errors."),
                ParserResult::NoMatch { expect } => panic!("Should always match."),
            }
        }
        _ => panic!(
            "Illegal primary expression {:?}",
            parser_struct.current_token
        ),
    }
}

// =================================== Parameters and Arguments ====================================

fn parse_parameter_list(parser_data: &mut ParserData) -> ListType {
    let start_pos = parser_data.current_token.token_position.clone();
    match_and_consume_next_token(parser_data); // Consume LPAREN

    if parser_data.current_token.token_kind != TokenKind::RPAREN {
        let proper_para_list = parse_proper_parameter_list(parser_data);
        match_and_consume_next_token(parser_data); // Consume RPAREN
        proper_para_list
    } else {
        match_and_consume_next_token(parser_data); // Consume RPAREN
        let final_pos = finish_position(&start_pos, &parser_data.current_token.token_position);
        ListType::EmptyParamList(EmptyParamList::new(final_pos))
    }
}

fn parse_proper_parameter_list(parser_data: &mut ParserData) -> ListType {
    let start_pos = parser_data.current_token.token_position.clone();
    let param_decl = parse_parameter_declaration(parser_data);

    match parser_data.current_token.token_kind {
        TokenKind::COMMA => {
            match_and_consume_next_token(parser_data);
            let para_list = parse_proper_parameter_list(parser_data);
            let final_pos = finish_position(&start_pos, &parser_data.current_token.token_position);

            ListType::ParamList(ParamList::new(final_pos, param_decl, Box::new(para_list)))
        }
        _ => {
            let empty_para_list = ListType::EmptyParamList(EmptyParamList::new(start_pos));
            ListType::ParamList(ParamList::new(
                start_pos,
                param_decl,
                Box::new(empty_para_list),
            ))
        }
    }
}

fn parse_parameter_declaration(parser_data: &mut ParserData) -> ParaDecl {
    let start_pos = parser_data.current_token.token_position;
    let param_type = parse_type(parser_data);
    let ident = parse_identifier(parser_data);

    match parser_data.current_token.token_kind {
        TokenKind::LBRACKET => {
            let type_variant = parse_declarator(parser_data, Box::new(param_type));
            let final_pos = finish_position(&start_pos, &parser_data.current_token.token_position);
            ParaDecl::new(final_pos, Box::new(type_variant), Box::new(ident))
        }
        _ => {
            let final_pos = finish_position(&start_pos, &parser_data.current_token.token_position);

            ParaDecl::new(
                final_pos,
                Box::new(AstTypeVariant::Primitive(param_type)),
                Box::new(ident),
            )
        }
    }
}

// arg-list -> "(" proper-arg-list? ")"
fn parse_argument_list(parser_struct: &mut ParserData) -> ListType {
    let start = parser_struct.current_position;
    match_and_consume_next_token(parser_struct); // Consume LPAREN

    if parser_struct.current_token.token_kind == TokenKind::RPAREN {
        let final_pos = finish_position(&start, &parser_struct.current_position);
        match_and_consume_next_token(parser_struct); // Consume RPAREN
        return ListType::EmptyArgList(EmptyArgList::new(final_pos));
    }

    let arg_list = parse_proper_argument_list(parser_struct);
    match_and_consume_next_token(parser_struct); // Consume RPAREN
    arg_list
}

// proper-arg-list -> arg ( "," arg )*
fn parse_proper_argument_list(parser_struct: &mut ParserData) -> ListType {
    let start_pos = parser_struct.current_position;
    let arg = parse_arg(parser_struct);

    let arg_expr = ExprType::Arg(arg);

    if parser_struct.current_token.token_kind != TokenKind::COMMA {
        let final_pos = finish_position(&start_pos, &parser_struct.current_position);
        // Shouldn't return empty Arg List.
        return ListType::ArgList(ArgList::new(
            final_pos,
            arg_expr,
            Box::new(ListType::EmptyArgList(EmptyArgList::new(final_pos))),
        ));
    }

    match_and_consume_next_token(parser_struct); // Consume COMMA
    parse_proper_argument_list(parser_struct)
}

fn parse_arg(parser_data: &mut ParserData) -> Arg {
    let arg_position = parser_data.current_token.token_position.clone();
    let expr = parse_expr(parser_data);
    Arg::new(arg_position, Box::new(expr))
}

// ========================== Literal and Type Parsing ==========================

fn parse_int_literal(parser_data: &mut ParserData) -> IntLiteral {
    match parser_data.current_token.token_kind {
        TokenKind::INTLITERAL => {
            let int_literal_node = IntLiteral {
                source_position: parser_data.current_token.token_position.clone(),
                spelling: parser_data.current_token.spelling.clone(),
            };

            match_and_consume_next_token(parser_data);
            int_literal_node
        }
        _ => panic!("integer literal expected here"),
    }
}

fn parse_float_literal(parser_data: &mut ParserData) -> FloatLiteral {
    match parser_data.current_token.token_kind {
        TokenKind::FLOATLITERAL => {
            let float_literal_node = FloatLiteral {
                source_position: parser_data.current_token.token_position.clone(),
                spelling: parser_data.current_token.spelling.clone(),
            };

            match_and_consume_next_token(parser_data);

            float_literal_node
        }
        _ => panic!("float literal expected here"),
    }
}

fn parse_boolean_literal(parser_data: &mut ParserData) -> BooleanLiteral {
    match parser_data.current_token.token_kind {
        TokenKind::BOOLEANLITERAL => {
            let boolean_literal_node = BooleanLiteral {
                source_position: parser_data.current_token.token_position.clone(),
                spelling: parser_data.current_token.spelling.clone(),
            };

            match_and_consume_next_token(parser_data);

            boolean_literal_node
        }
        _ => panic!("boolean literal expected here"),
    }
}

fn parse_string_literal(parser_struct: &mut ParserData) -> ParserResult {
    match parser_struct.current_token.token_kind {
        TokenKind::STRINGLITERAL => {
            let string_literal_node = StringLiteral {
                source_position: parser_struct.current_token.token_position.clone(),
                spelling: parser_struct.current_token.spelling.clone(),
            };
            // Here, we assume that scanner.get_next_token() updates
            // parser_struct.current_token with the next token and returns it.

            match_and_consume_next_token(parser_struct);

            ParserResult::Ok(AstNode::StringLiteral(string_literal_node))
        }
        _ => ParserResult::NoMatch {
            expect: "string literal",
        },
    }
}

fn parse_identifier(parser_struct: &mut ParserData) -> Ident {
    match parser_struct.current_token.token_kind {
        TokenKind::ID => {
            let identifier = Ident::new(
                parser_struct.current_token.spelling.clone(),
                parser_struct.current_token.token_position,
                None,
            );
            match_and_consume_next_token(parser_struct);
            identifier
        }
        _ => panic!("identifier expected here"),
    }
}

fn parse_type(parser_data: &mut ParserData) -> AstTypes {
    let type_position = parser_data.current_token.token_position.clone();

    match parser_data.current_token.token_kind {
        TokenKind::VOID => {
            match_and_consume_next_token(parser_data);
            AstTypes::VoidType(VoidType::new(type_position))
        }
        TokenKind::INT => {
            match_and_consume_next_token(parser_data);
            AstTypes::IntType(IntType::new(type_position))
        }
        TokenKind::FLOAT => {
            match_and_consume_next_token(parser_data);
            AstTypes::FloatType(FloatType::new(type_position))
        }
        TokenKind::BOOLEAN => {
            match_and_consume_next_token(parser_data);
            AstTypes::BooleanType(BooleanType::new(type_position))
        }
        TokenKind::STRINGLITERAL => {
            match_and_consume_next_token(parser_data);
            AstTypes::StringType(StringType::new(type_position))
        }
        _ => panic!(
            "\"{:?}\" wrong type. Type expected.",
            parser_data.current_token
        ),
    }
}

// =================================== Utility Functions =====================================

fn match_and_consume_next_token(parser_struct: &mut ParserData) {
    parser_struct.current_token = parser_struct.scanner.get_next_token();
    parser_struct.current_position = parser_struct.current_token.token_position;
}

fn consume_operator(parser_struct: &mut ParserData) -> Operator {
    let operator = Operator::new(
        parser_struct.current_token.token_position.clone(),
        parser_struct.current_token.spelling.clone(),
    );
    parser_struct.current_token = parser_struct.scanner.get_next_token();
    parser_struct.current_position = parser_struct.current_token.token_position;
    operator
}

fn accept_token(parser_struct: &mut ParserData, token: Token) -> bool {
    todo!("Complete accept token");
}

fn finish_position(
    init_source_position: &SourcePosition,
    end_source_position: &SourcePosition,
) -> SourcePosition {
    // Take the start of the initial and the end of the end and create a single source position.
    SourcePosition::new(
        init_source_position.line_start,
        end_source_position.line_finish,
        init_source_position.char_start,
        end_source_position.char_end,
    )
}

static EXPR_FIRST_SET: Lazy<HashSet<TokenKind>> = Lazy::new(|| {
    [
        TokenKind::ID,
        TokenKind::INTLITERAL,
        TokenKind::FLOATLITERAL,
        TokenKind::BOOLEANLITERAL,
        TokenKind::STRINGLITERAL,
        TokenKind::PLUS,
        TokenKind::MINUS,
        TokenKind::NOT,
        TokenKind::MULT,
        TokenKind::DIV,
        TokenKind::LPAREN,
        TokenKind::SEMICOLON,
    ]
    .iter()
    .cloned()
    .collect::<HashSet<_>>()
});

fn is_primitive_type(ast_type: &AstTypes) -> bool {
    match ast_type {
        AstTypes::IntType(_) => true,
        AstTypes::FloatType(_) => true,
        AstTypes::BooleanType(_) => true,
        AstTypes::VoidType(_) => true,
        _ => false,
    }
}

fn token_is_primitive_type(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::INT => true,
        TokenKind::FLOAT => true,
        TokenKind::BOOLEAN => true,
        TokenKind::VOID => true,
        _ => false,
    }
}
