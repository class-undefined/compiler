use koopa::front::driver;
use koopa::ir::builder_traits::*;
use koopa::ir::*;

use super::stmt::Stmt;

#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt,
}

impl Block {
    pub fn to_ir(&self) -> String {
        format!("\n%entry: \n  {}\n", self.stmt.to_ir())
    }
}
