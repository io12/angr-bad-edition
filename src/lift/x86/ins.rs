use crate::ir;

use std::rc::Rc;

use capstone::arch::x86::X86Operand;
use capstone::arch::x86::X86OperandType;
use capstone::prelude::*;

fn lift_set_dst(dst: &X86OperandType, cs: &Capstone) -> ir::SetDst {
    match dst {
        X86OperandType::Reg(reg_id) => ir::SetDst::Reg {
            name: cs.reg_name(*reg_id).expect("invalid register id"),
        },
        _ => unimplemented!(),
    }
}

fn lift_read_operand(operand: &X86OperandType, cs: &Capstone) -> ir::Expr {
    match operand {
        X86OperandType::Reg(reg_id) => ir::Expr::Reg {
            name: cs.reg_name(*reg_id).expect("invalid register id"),
        },
        X86OperandType::Imm(val) => ir::Expr::Const { val: *val },
        _ => unimplemented!(),
    }
}

pub fn lift_xor(operands: &[X86Operand], cs: &Capstone) -> ir::Stmt {
    if let [dst, src] = operands {
        let dst = &dst.op_type;
        let src = &src.op_type;
        ir::Stmt::Set {
            dst: lift_set_dst(dst, cs),
            val: ir::Expr::BinOp {
                kind: ir::BinOpKind::Xor,
                left: Rc::new(lift_read_operand(dst, cs)),
                right: Rc::new(lift_read_operand(src, cs)),
            },
        }
    } else {
        panic!("invalid amount of operands")
    }
}
