use std::fmt::{Debug, Error, Formatter};

#[derive(Copy, Clone)]
pub enum OpCode {
    Add,
    Sub,
    Mul,
    Div,
    Pos,
    Neg,
    Not,
}

impl Debug for OpCode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match &self {
            OpCode::Mul => write!(fmt, "*"),
            OpCode::Div => write!(fmt, "/"),
            OpCode::Add => write!(fmt, "+"),
            OpCode::Sub => write!(fmt, "-"),
            OpCode::Not => write!(fmt, "!"),
            OpCode::Pos => write!(fmt, "+"),
            OpCode::Neg => write!(fmt, "-"),
        }
    }
}

pub enum Expr {
    Number(i32),
    UnaryOp(OpCode, Box<Expr>),          // 一元表达,
    BinOp(Box<Expr>, OpCode, Box<Expr>), // 二元表达
}

impl Expr {
    pub fn exec(&self) -> i32 {
        match &self {
            Self::Number(num) => num.clone(),
            Self::BinOp(lhs, op, rhs) => {
                let left = lhs.exec();
                let right = rhs.exec();
                let value = match op {
                    OpCode::Add => left + right,
                    OpCode::Sub => left - right,
                    OpCode::Mul => left * right,
                    OpCode::Div => {
                        if right == 0 {
                            panic!("cant divide by zero");
                        }
                        left / right
                    }
                    _ => panic!("error"),
                };
                value
            }
            Self::UnaryOp(op, rhs) => match op {
                OpCode::Not => !rhs.exec(),
                OpCode::Pos => rhs.exec(),
                OpCode::Neg => -rhs.exec(),
                _ => panic!("error"),
            },
        }
    }
    pub fn to_ir(&self) -> i32 {
        self.exec()
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match &self {
            Expr::Number(num) => write!(fmt, "{}", num),
            Expr::BinOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Expr::UnaryOp(op, ref r) => write!(fmt, "({:?} {:?})", op, r),
        }
    }
}
