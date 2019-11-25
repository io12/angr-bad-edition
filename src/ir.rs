use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

/// A register (contains register name)
// TODO: make this less dynamic
#[derive(Debug, Clone)]
pub struct Reg(pub String);

impl Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "regs.{}", self.0)
    }
}

/// A block of statements in the IR
#[derive(Debug, Clone)]
pub struct Block(pub Vec<Stmt>);

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.0 {
            write!(f, "{}\n", stmt)?;
        }
        Ok(())
    }
}

/// A statement in the IR
#[derive(Debug, Clone)]
pub enum Stmt {
    /// Assignment
    Set {
        /// Destination
        dst: SetDst,

        /// Value to assign
        val: Expr,
    },
}

impl Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Set { dst, val } => write!(f, "{} = {};", dst, val),
        }
    }
}

/// Destination of set statement
#[derive(Debug, Clone)]
pub enum SetDst {
    /// Register assignment
    Reg(Reg),

    /// Assignment to memory address
    Mem(Expr),
}

impl Display for SetDst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetDst::Reg(reg) => write!(f, "{}", reg),
            SetDst::Mem(addr) => write!(f, "mem[{}]", addr),
        }
    }
}

/// An expression in the IR
#[derive(Debug, Clone)]
pub enum Expr {
    /// A binary operation
    BinOp {
        /// Kind of operation
        kind: BinOpKind,

        /// Left operand
        left: Rc<Expr>,

        /// Right operand
        right: Rc<Expr>,
    },

    /// A constant value
    Const(i64),

    /// A register value
    Reg(Reg),
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::BinOp { kind, left, right } => write!(f, "({} {} {})", left, kind, right),
            Expr::Const(val) => write!(f, "{:#x}", val),
            Expr::Reg(reg) => write!(f, "{}", reg),
        }
    }
}

/// A kind of binary operation
#[derive(Debug, Clone)]
pub enum BinOpKind {
    Add,
    Xor,
}

impl Display for BinOpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOpKind::Add => f.write_str("+"),
            BinOpKind::Xor => f.write_str("^"),
        }
    }
}
