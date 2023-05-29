//use std::any::Any;
//
//// Visitor
//pub trait Visitor {
//    // Programs
//    fn visit_program(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//
//    // Lists for denoting the null reference
//    fn visit_empty_decl_list(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_empty_stmt_list(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_empty_array_expr_list(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_empty_para_list(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_empty_arg_list(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//
//    // Declarations
//    fn visit_decl_list(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_func_decl(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_global_var_decl(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_local_var_decl(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//
//    // Stmts
//    fn visit_stmt_list(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    // Dyn, any type that implements that trait.
//    fn visit_if_stmt(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_while_stmt(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_for_stmt(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_break_stmt(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_continue_stmt(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_return_stmt(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_compound_stmt(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_expr_stmt(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_empty_comp_stmt(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_empty_stmt(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//
//    // Expressions
//    fn visit_int_expr(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_float_expr(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_boolean_expr(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_string_expr(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any;
//
//    // Literals and identifiers
//    fn visit_int_literal(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_float_literal(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_boolean_literal(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_string_literal(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_ident(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_operator(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//
//    // Parameters
//    fn visit_para_list(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_para_decl(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//
//    // Arguments
//    fn visit_arg_list(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_arg(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//
//    // Types
//    fn visit_void_type(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_boolean_type(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_int_type(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_float_type(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_string_type(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_array_type(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//    fn visit_error_type(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//
//    // Variables
//    fn visit_simple_var(&self, ast: &dyn AST, o: &dyn Any) -> Box<dyn Any>;
//}
