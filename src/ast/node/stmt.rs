use koopa::front::driver;
use koopa::ir::builder_traits::*;
use koopa::ir::*;

use super::Expr;

#[derive(Debug)]
pub struct Stmt {
    pub expr: Box<Expr>,
}

impl Stmt {
    pub fn to_ir(&self) -> String {
        println!("{:?}", self.expr);
        format!("ret {}", self.expr.to_ir())
    }
}
