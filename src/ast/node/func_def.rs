use koopa::front::driver;
use koopa::ir::*;

use super::block::Block;

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {
    Int,
}

impl FuncType {
    pub fn to_ir(&self) -> String {
        match self {
            FuncType::Int => "i32".to_string(),
        }
    }
}

impl FuncDef {
    pub fn to_ir(&self) -> String {
        format!(
            "fun @{}(): {} {{{}}}",
            self.ident,
            self.func_type.to_ir(),
            self.block.to_ir()
        )
    }

    pub fn memory(&self) -> Program {
        let s = self.to_ir();
        let driver = driver::Driver::from(s);
        driver.generate_program().unwrap()
    }
}
