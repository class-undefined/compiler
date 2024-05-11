use koopa::front::driver;
use koopa::ir::builder_traits::*;
use koopa::ir::*;
#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

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

#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt,
}

#[derive(Debug)]
pub struct Stmt {
    pub num: i32,
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

impl Block {
    pub fn to_ir(&self) -> String {
        format!("\n%entry: \n  {}\n", self.stmt.to_ir())
    }
}

impl Stmt {
    pub fn to_ir(&self) -> String {
        format!("ret {}", self.num)
    }
}
