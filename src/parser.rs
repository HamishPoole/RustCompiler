// Parser consumes tokens, adds them to concrete syntax tree.
// Visitor pattern creates abstract Syntax tree from concrete syntax tree.
// Can visualise using https://crates.io/crates/graphviz-rust https://docs.rs/petgraph/latest/petgraph/ if needed.
// Arc<Mutex<T>> or RC or RefCell for references.

use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::ast::decl::{DeclType, FuncDecl, ParaDecl};
use crate::ast::empty_expr::EmptyExpr;
use crate::ast::expression::{
    Arg, ArrayInitExpr, AssignExpr, BinaryExpr, ExprType, StringExpr, UnaryExpr,
};
use crate::ast::ident::Ident;
use crate::ast::list::{
    ArrayExprList, DeclList, EmptyArrayExprList, EmptyParamList, ListType, ParamList,
};
use crate::ast::list::{EmptyDeclList, EmptyStmtList, StmtList};
use crate::ast::literals::{BooleanLiteral, FloatLiteral, IntLiteral, Operator, StringLiteral};
use crate::ast::primitive_types::{
    BooleanType, FloatType, IntType, PrimitiveType, StringType, VoidType,
};
use crate::ast::statement::{
    BreakStmt, CompoundStmt, ContinueStmt, EmptyStmt, ExprStmt, ReturnStmt, StmtType, WhileStmt,
};
use crate::ast::{expression, AstNode};
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

pub struct ParserData {
    scanner: Scanner,
    current_token: Token,
    current_position: SourcePosition,
}

// NB.  No errors should be recoverable in this parser.
// Recoverable errors imply the parser should handle it.  Syntax errors are not recoverable.
impl ParserData {
    pub fn new(mut scanner: Scanner) -> Self {
        Self {
            scanner,
            current_token: Token::new(
                TokenKind::ERROR,
                TokenKind::ERROR.to_string(),
                SourcePosition::new(0, 0, 0, 0),
            ),
            current_position: SourcePosition::new(0, 0, 0, 0),
        }
    }
}

pub fn parse_program(scanner: Scanner) -> AstNode {
    let mut parser = ParserData::new(scanner);
    generate_ast(&mut parser)
}
pub fn generate_ast(parser_struct: &mut ParserData) -> AstNode {
    let declaration_list = parse_declaration_list(parser_struct, true);

    todo!("Complete parse program");
}

fn parse_declaration_list(parser_struct: &mut ParserData, is_global: bool) -> ListType {
    todo!("Complete parse declaration list");
}

// fn parse_func_decl_list(
//     type_: Type,
//     ident: Ident,
//     is_global: bool,
//     parser_struct: &mut ParserData,
// ) -> DeclList {
//     let decl_list_pos = start_source_position(parser_struct);
//
//     let lhs_child = parse_func_decl(type_, ident, parser_struct);
//
//     let mut right_child_decl_list = match parser_struct.current_token.kind {
//         Token::EOF => EmptyDeclList::new(dummy_pos),
//         _ => parse_declare_list(is_global, parser_struct),
//     };
//
//     finish_source_position(&mut decl_list_pos, parser_struct);
//
//     DeclList::new(lhs_child, right_child_decl_list, decl_list_pos)
//
//     todo!("Complete parse function declaration list");
// }

fn parse_func_decl(
    function_type: Box<PrimitiveType>,
    ident: Ident,
    parser_struct: &mut ParserData,
) -> FuncDecl {
    let start_pos = parser_struct.current_position;

    let function_parameter_list_ast = parse_parameter_list(parser_struct);
    let compound_stmt_ast = parse_compound_stmt(parser_struct);

    let final_pos = finish_source_pos(&start_pos, &parser_struct.current_position);

    FuncDecl::new(
        final_pos,
        function_type,
        Box::new(ident),
        Box::new(function_parameter_list_ast),
        Box::new(compound_stmt_ast),
    )
}

// init-declarator-list-> init-declarator ( "," init-declarator )*
fn parse_initial_declaration_list(
    parser_struct: &mut ParserData,
    decl_type: Box<PrimitiveType>,
    identifier: Box<Ident>,
    is_global: bool,
) -> ListType {
    todo!("Complete parse initial declaration list");
}

// init-declarator -> declarator ( "=" initializer )?
fn parse_initial_array_declarator(
    parser_struct: &mut ParserData,
    decl_type: Box<PrimitiveType>,
) -> DeclType {
    todo!("Complete parse initial declarator");
}

// declarator -> identifier
// | identifier "[" INTLITERAL? "]"
fn parse_array_declarator(
    parser_struct: &mut ParserData,
    decl_type: Box<PrimitiveType>,
) -> PrimitiveType {
    todo!("Complete parse declarator");
}

// initialiser -> expr
// | "{" expr ( "," expr )* "}"
fn parse_array_initialiser(parser_struct: &mut ParserData) -> ExprType {
    let start_pos = parser_struct.current_position;

    match parser_struct.current_token.token_kind {
        TokenKind::LBRACE => {
            match_and_consume_next_token(parser_struct); // Consume LCURLY.
            let array_expr_list = parse_array_expr_list(parser_struct);
            let finish_pos = finish_source_pos(&start_pos, &parser_struct.current_position);
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
            let finish_pos = finish_source_pos(&start_pos, &parser_struct.current_position);

            ListType::ArrayExprList(ArrayExprList::new(
                finish_pos,
                lhs_expr,
                Box::new(rhs_array_expr),
            ))
        }
        _ => {
            let finish_pos = finish_source_pos(&start_pos, &parser_struct.current_position);
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

fn parse_ident(parser_struct: &mut ParserData) -> Ident {
    match parser_struct.current_token.token_kind {
        TokenKind::ID => {
            let identifier = Ident::new(
                parser_struct.current_token.spelling.clone(),
                parser_struct.current_token.token_position.clone(),
                None,
            );
            match_and_consume_next_token(parser_struct);
            identifier
        }
        _ => panic!("identifier expected here"),
    }
}

// compound-stmt -> "{" var-decl* stmt* "}"
fn parse_compound_stmt(parser_struct: &mut ParserData) -> StmtType {
    let start_pos = parser_struct.current_position;

    // Consumes LCURLY.
    match_and_consume_next_token(parser_struct);

    let mut declare_list = ListType::EmptyDeclList(EmptyDeclList::new(start_pos));

    if is_primitive_type(&parser_struct.current_token.token_kind) {
        declare_list = parse_declaration_list(parser_struct, false);
    }

    let stmt_list_ast = parse_stmt_list(parser_struct);

    // Consumers RCURLY
    match_and_consume_next_token(parser_struct);

    let final_source_pos = finish_source_pos(&start_pos, &parser_struct.current_position);

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
    let final_source_pos = finish_source_pos(&start_pos, &parser_struct.current_position.clone());

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
        TokenKind::IF => parse_expr_stmt(parser_struct),
        TokenKind::FOR => parse_expr_stmt(parser_struct),
        TokenKind::WHILE => parse_while_stmt(parser_struct),
        TokenKind::BREAK => parse_break_stmt(parser_struct),
        TokenKind::CONTINUE => parse_continue_stmt(parser_struct),
        TokenKind::RETURN => parse_return_stmt(parser_struct),
        _ => {
            if EXPR_FIRST_SET.contains(&parser_struct.current_token.token_kind) {
                parse_expr_stmt(parser_struct)
            } else {
                panic!("Invalid token kind for single statement");
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
    todo!("Complete parse if statement");
}

// for-stmt -> for "(" expr? ";" expr? ";" expr? ")" stmt
fn parse_for_statement(parser_struct: &mut ParserData) -> StmtType {
    todo!("Complete parse for statement");
}

// while-stmt -> while "(" expr ")" stmt
fn parse_while_stmt(parser_struct: &mut ParserData) -> StmtType {
    let start_pos = parser_struct.current_token.token_position;

    match_and_consume_next_token(parser_struct); // Consume TokenKind::WHILE
    match_and_consume_next_token(parser_struct); // Consume TokenKind::LPAREN

    let while_condition_ast = Box::new(parse_expr(parser_struct));

    match_and_consume_next_token(parser_struct); // Consume TokenKind::RPAREN

    let while_stmt = parse_single_or_multiple_statements(parser_struct);

    let final_position = finish_source_pos(
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

    let final_position = finish_source_pos(
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

    // Consume TokenKind::RETURN
    match_and_consume_next_token(parser_struct);

    if EXPR_FIRST_SET.contains(&parser_struct.current_token.token_kind) {
        let expr = Box::new(parse_expr(parser_struct));
        let final_position = finish_source_pos(
            &start_pos,
            &parser_struct.current_token.token_position.clone(),
        );
        return StmtType::ReturnStmt(ReturnStmt::new(final_position, expr));
    };

    // Consume TokenKind::SEMICOLON
    match_and_consume_next_token(parser_struct);
    let final_position = finish_source_pos(
        &start_pos,
        &parser_struct.current_token.token_position.clone(),
    );
    let empty_expr = ExprType::EmptyExpr(EmptyExpr::new(final_position));

    StmtType::ReturnStmt(ReturnStmt::new(final_position, Box::new(empty_expr)))
}

// expr-stmt -> expr? ";"
fn parse_expr_stmt(parser_struct: &mut ParserData) -> StmtType {
    let mut assign_expr_pos = parser_struct.current_token.token_position;

    match parser_struct.current_token.token_kind {
        kind if EXPR_FIRST_SET.contains(&kind) => {
            let expr_ast = parse_expr(parser_struct);
            match_and_consume_next_token(parser_struct);
            let final_source_pos = finish_source_pos(
                &assign_expr_pos,
                &parser_struct.current_token.token_position.clone(),
            );
            let expr_stmt = ExprStmt::new(final_source_pos, expr_ast);
            StmtType::ExprStmt(expr_stmt)
        }
        TokenKind::SEMICOLON => {
            match_and_consume_next_token(parser_struct);
            let final_source_pos = finish_source_pos(
                &assign_expr_pos,
                &parser_struct.current_token.token_position.clone(),
            );

            let empty_expr = ExprType::EmptyExpr(EmptyExpr::new(final_source_pos));
            StmtType::ExprStmt(ExprStmt::new(final_source_pos, empty_expr))
        }
        _ => panic!("Error in expr statement."),
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
    let mut assign_expr_pos = parser_struct.current_token.token_position.clone();
    let mut lhs_expr = parse_conditional_or_expression(parser_struct);

    match parser_struct.current_token.token_kind {
        TokenKind::EQ => {
            let eq = consume_operator(parser_struct);
            let rhs = parse_assign_expr(parser_struct);
            let assign_expr = AssignExpr::new(
                finish_source_pos(&assign_expr_pos, &rhs.get_source_position()),
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
                finish_source_pos(start_pos, &second_expr.get_source_position()),
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
    main_expr = parse_conditional_and_expression_tail(&cond_and_expr_pos, main_expr, parser_struct);
    main_expr
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
                finish_source_pos(start_pos, &second_expr.get_source_position()),
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
    lhs_expr = parse_equality_expression_tail(&equality_expr_pos, lhs_expr, parser_struct);
    lhs_expr
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
                finish_source_pos(start_pos, &rhs_expr.get_source_position()),
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
    let rel_expr = parse_relational_expression_tail(&rel_expr_pos, lhs_expr, parser_struct);
    rel_expr
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
                finish_source_pos(start_pos, &rhs_expr.get_source_position()),
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
    let main_expr = parse_additive_expression_tail(&pos, lhs_expr, parser_struct);
    main_expr
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
                finish_source_pos(start_pos, &rhs_expr.get_source_position()),
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
    let main_expr = parse_multiplicative_expression_tail(&start_pos, lhs_expr, parser_struct);
    main_expr
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
                finish_source_pos(start_pos, &rhs_expr.get_source_position()),
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
                finish_source_pos(&start_pos, &lhs_expr.get_source_position()),
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
    match parser_struct.current_token.token_kind {
        // TokenKind::ID => {
        //     let lhs_ident = parse_ident(parser_struct)?; // Assuming parse_ident is implemented and returns a Result
        //     let sim_vast = SimpleVar { lhs_ident, primary_pos };
        //
        //     match parser_struct.current_token.token_kind {
        //         TokenKind::LBRACKET => {
        //             consume_token(TokenKind::LBRACKET, parser_struct)?; // Assuming consume_token is implemented and returns a Result
        //             let rhs_expr = parse_expr(parser_struct)?; // Assuming parse_expr is implemented and returns a Result
        //             consume_token(TokenKind::RBRACKET, parser_struct)?;
        //             ParserResult::Ok(AstNode::ArrayExpr(ArrayExpr { sim_vast, rhs_expr, primary_pos }))
        //         },
        //         TokenKind::LPAREN => {
        //             let args = parse_arg_list(parser_struct)?; // Assuming parse_arg_list is implemented and returns a Result
        //             ParserResult::Ok(AstNode::CallExpr(CallExpr { lhs_ident, args, primary_pos }))
        //         },
        //         _ => {
        //             ParserResult::Ok(AstNode::VarExpr(VarExpr { sim_vast, primary_pos }))
        //         },
        //     }
        // },
        // TokenKind::LPAREN => {
        //     consume_token(TokenKind::LPAREN, parser_struct)?;
        //     let expr_ast = parse_expr(parser_struct)?;
        //     consume_token(TokenKind::RPAREN, parser_struct)?;
        //     ParserResult::Ok(expr_ast)
        // },
        // TokenKind::INTLITERAL => {
        //     let int_literal = parse_int_literal(parser_struct)?;
        //     ParserResult::Ok(AstNode::IntExpr(IntExpr { int_literal, primary_pos }))
        // },
        // TokenKind::FLOATLITERAL => {
        //     let float_literal = parse_float_literal(parser_struct)?;
        //     ParserResult::Ok(AstNode::FloatExpr(FloatExpr { float_literal, primary_pos }))
        // },
        // TokenKind::BOOLEANLITERAL => {
        //     let bool_literal = parse_boolean_literal(parser_struct)?;
        //     ParserResult::Ok(AstNode::BooleanExpr(BooleanExpr { bool_literal, primary_pos }))
        // },
        TokenKind::STRINGLITERAL => {
            let string_literal = parse_string_literal(parser_struct);
            match string_literal {
                ParserResult::Ok(ast_node) => match ast_node {
                    AstNode::StringLiteral(string_lit) => {
                        // Note how doing it in one line made ownership pass easily.
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
        _ => panic!("Illegal primary expression"),
    }
}

fn parse_parameter_list(parser_data: &mut ParserData) -> ListType {
    let start_pos = parser_data.current_token.token_position.clone();

    match_and_consume_next_token(parser_data); // Consume LPAREN

    if parser_data.current_token.token_kind != TokenKind::RPAREN {
        let proper_para_list = parse_proper_parameter_list(parser_data);
        match_and_consume_next_token(parser_data); // Consume RPAREN
        proper_para_list
    } else {
        match_and_consume_next_token(parser_data); // Consume RPAREN
        let final_pos = finish_source_pos(&start_pos, &parser_data.current_token.token_position);
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
            let final_pos =
                finish_source_pos(&start_pos, &parser_data.current_token.token_position);

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
    let ident = parse_ident(parser_data);

    match parser_data.current_token.token_kind {
        TokenKind::LBRACKET => {
            let array_type = parse_array_declarator(parser_data, Box::new(param_type));
            let final_pos =
                finish_source_pos(&start_pos, &parser_data.current_token.token_position);
            ParaDecl::new(final_pos, Box::new(array_type), Box::new(ident))
        }
        _ => {
            let final_pos =
                finish_source_pos(&start_pos, &parser_data.current_token.token_position);

            ParaDecl::new(final_pos, Box::new(param_type), Box::new(ident))
        }
    }
}

fn parse_argument_list(parser_struct: &mut ParserData) -> AstNode {
    todo!("Complete parse argument list");
}

fn parse_proper_argument_list(parser_struct: &mut ParserData) -> AstNode {
    todo!("Complete parse proper argument list");
}

fn parse_arg(parser_data: &mut ParserData) -> AstNode {
    let arg_position = parser_data.current_token.token_position.clone();
    let expr = parse_expr(parser_data);
    AstNode::Arg(Arg::new(arg_position, Box::new(expr)))
}

fn parse_int_literal(parser_data: &mut ParserData) -> AstNode {
    match parser_data.current_token.token_kind {
        TokenKind::INTLITERAL => {
            let int_literal_node = IntLiteral {
                source_position: parser_data.current_token.token_position.clone(),
                spelling: parser_data.current_token.spelling.clone(),
            };

            match_and_consume_next_token(parser_data);

            AstNode::IntLiteral(int_literal_node)
        }
        _ => panic!("integer literal expected here"),
    }
}

fn parse_float_literal(parser_data: &mut ParserData) -> AstNode {
    match parser_data.current_token.token_kind {
        TokenKind::FLOATLITERAL => {
            let float_literal_node = FloatLiteral {
                source_position: parser_data.current_token.token_position.clone(),
                spelling: parser_data.current_token.spelling.clone(),
            };

            match_and_consume_next_token(parser_data);

            AstNode::FloatLiteral(float_literal_node)
        }
        _ => panic!("float literal expected here"),
    }
}

fn parse_boolean_literal(parser_data: &mut ParserData) -> AstNode {
    match parser_data.current_token.token_kind {
        TokenKind::BOOLEANLITERAL => {
            let boolean_literal_node = BooleanLiteral {
                source_position: parser_data.current_token.token_position.clone(),
                spelling: parser_data.current_token.spelling.clone(),
            };

            match_and_consume_next_token(parser_data);

            AstNode::BooleanLiteral(boolean_literal_node)
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

fn parse_type(parser_data: &mut ParserData) -> PrimitiveType {
    let type_position = parser_data.current_token.token_position.clone();

    match parser_data.current_token.token_kind {
        TokenKind::VOID => {
            match_and_consume_next_token(parser_data);
            PrimitiveType::VoidType(VoidType::new(type_position))
        }
        TokenKind::INT => {
            match_and_consume_next_token(parser_data);
            PrimitiveType::IntType(IntType::new(type_position))
        }
        TokenKind::FLOAT => {
            match_and_consume_next_token(parser_data);
            PrimitiveType::FloatType(FloatType::new(type_position))
        }
        TokenKind::BOOLEAN => {
            match_and_consume_next_token(parser_data);
            PrimitiveType::BooleanType(BooleanType::new(type_position))
        }
        TokenKind::STRINGLITERAL => {
            match_and_consume_next_token(parser_data);
            PrimitiveType::StringType(StringType::new(type_position))
        }
        _ => panic!(
            "\"{}\" wrong type. Type expected.",
            parser_data.current_token.spelling
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
fn finish_source_pos(
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
    ]
    .iter()
    .cloned()
    .collect::<HashSet<_>>()
});

fn is_primitive_type(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::INT => true,
        TokenKind::FLOAT => true,
        TokenKind::BOOLEAN => true,
        TokenKind::STRINGLITERAL => true,
        _ => false,
    }
}
