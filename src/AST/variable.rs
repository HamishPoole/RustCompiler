use crate::ast::primitive_types::PrimitiveType;
use crate::utils::SourcePosition;

#[derive(Clone, Debug)]
pub struct Var {
    source_position: SourcePosition,
    var_type: Box<PrimitiveType>,
}
