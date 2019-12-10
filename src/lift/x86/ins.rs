use crate::ir;

use std::rc::Rc;

use capstone::arch::x86::X86Operand;
use capstone::arch::x86::X86OperandType;
use capstone::prelude::*;

const WORD_SIZE: i64 = 4;

fn lift_set_dst(dst: &X86OperandType, cs: &Capstone) -> ir::SetDst {
    match dst {
        X86OperandType::Reg(reg_id) => {
            let name = cs.reg_name(*reg_id).expect("invalid register id");
            ir::SetDst::Reg(ir::Reg(name))
        }
        _ => unimplemented!("{:#?}", dst),
    }
}

fn lift_read_operand(operand: &X86OperandType, cs: &Capstone) -> ir::Expr {
    match operand {
        X86OperandType::Reg(reg_id) => {
            let name = cs.reg_name(*reg_id).expect("invalid register id");
            ir::Expr::Reg(ir::Reg(name))
        }
        X86OperandType::Imm(val) => ir::Expr::Const(*val),
        _ => unimplemented!("{:#?}", operand),
    }
}

pub fn lift_xor(operands: &[X86Operand], cs: &Capstone) -> ir::Block {
    if let [dst, src] = operands {
        let dst = &dst.op_type;
        let src = &src.op_type;
        ir::Block(vec![ir::Stmt::Set {
            dst: lift_set_dst(dst, cs),
            val: ir::Expr::BinOp {
                kind: ir::BinOpKind::Xor,
                left: Rc::new(lift_read_operand(dst, cs)),
                right: Rc::new(lift_read_operand(src, cs)),
            },
        }])
    } else {
        panic!("invalid amount of operands")
    }
}

pub fn lift_push(operands: &[X86Operand], cs: &Capstone) -> ir::Block {
    if let [operand] = operands {
        let operand = &operand.op_type;
        ir::Block(vec![
            ir::Stmt::Set {
                dst: ir::SetDst::Reg(ir::Reg("esp".into())),
                val: ir::Expr::BinOp {
                    kind: ir::BinOpKind::Add,
                    left: Rc::new(ir::Expr::Reg(ir::Reg("esp".into()))),
                    right: Rc::new(ir::Expr::Const(WORD_SIZE)),
                },
            },
            ir::Stmt::Set {
                dst: ir::SetDst::Mem(ir::Expr::Reg(ir::Reg("esp".into()))),
                val: lift_read_operand(operand, cs),
            },
        ])
    } else {
        panic!("invalid amount of operands")
    }
}

pub fn lift_mov(operands: &[X86Operand], cs: &Capstone) -> ir::Block {
    if let [dst, src] = operands {
        let dst = &dst.op_type;
        let src = &src.op_type;
        ir::Block(vec![ir::Stmt::Set {
            dst: lift_set_dst(dst, cs),
            val: lift_read_operand(src, cs),
        }])
    } else {
        panic!("invalid amount of operands")
    }
}
