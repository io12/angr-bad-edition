use std::rc::Rc;

/// A block of statements in the IR
#[derive(Debug)]
pub struct Block {
    /// Statements in block
    pub stmts: Vec<Stmt>,
}

/// A statement in the IR
#[derive(Debug)]
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
#[derive(Debug)]
pub enum SetDst {
    /// Register assignment
    Reg {
        // TODO: make this less dynamic
        /// Name of register
        name: String,
    },
}

/// An expression in the IR
#[derive(Debug)]
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
    Const {
        /// Value of expression
        val: i64,
    },

    /// A register value
    Reg {
        // TODO: make this less dynamic
        /// Name of register
        name: String,
    },
}

/// A kind of binary operation
#[derive(Debug)]
pub enum BinOpKind {
    Xor,
}
