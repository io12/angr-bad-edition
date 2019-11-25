use std::rc::Rc;

/// A block of statements in the IR
#[derive(Debug, Clone)]
pub struct Block(pub Vec<Stmt>);

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

/// Destination of set statement
#[derive(Debug, Clone)]
pub enum SetDst {
    /// Register assignment (contains register name)
    // TODO: make this less dynamic
    Reg(String),

    /// Assignment to memory address
    Mem(Expr),
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

    /// A register value (contains register name)
    // TODO: make this less dynamic
    Reg(String),
}

/// A kind of binary operation
#[derive(Debug, Clone)]
pub enum BinOpKind {
    Add,
    Xor,
}
