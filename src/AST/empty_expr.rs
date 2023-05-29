use crate::utils::SourcePosition;

#[derive(Clone, Debug)]
pub struct EmptyExpr {
    source_position: SourcePosition,
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
