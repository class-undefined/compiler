use koopa::front::driver;
use koopa::ir::builder_traits::*;
use koopa::ir::*;

#[derive(Debug)]
pub struct Stmt {
    pub num: i32,
}

impl Stmt {
    pub fn to_ir(&self) -> String {
        format!("ret {}", self.num)
    }
}
