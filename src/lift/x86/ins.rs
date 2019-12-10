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
        X86OperandType::Mem(mem) => {
            let addr = mem.disp();
            let addr = ir::Expr::Const(addr);
            ir::SetDst::Mem(addr)
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
        X86OperandType::Mem(mem) => {
            let disp = mem.disp();
            let base_reg = Rc::new(ir::Expr::Reg(ir::Reg(
                cs.reg_name(mem.base()).expect("invalid base register"),
            )));
            let op = if disp < 0 {
                ir::BinOpKind::Sub
            } else {
                ir::BinOpKind::Add
            };
            let disp = disp.abs();
            let disp = Rc::new(ir::Expr::Const(disp));
            let addr = ir::Expr::BinOp {
                kind: op,
                left: base_reg,
                right: disp,
            };
            ir::Expr::Mem(Rc::new(addr))
        }
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
                    kind: ir::BinOpKind::Sub,
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

pub fn lift_pop(operands: &[X86Operand], cs: &Capstone) -> ir::Block {
    if let [operand] = operands {
        let operand = &operand.op_type;
        ir::Block(vec![
            ir::Stmt::Set {
                dst: lift_set_dst(operand, cs),
                val: {
                    let esp = "esp".into();
                    let esp = ir::Reg(esp);
                    let esp = ir::Expr::Reg(esp);
                    let esp = Rc::new(esp);
                    ir::Expr::Mem(esp)
                },
            },
            ir::Stmt::Set {
                dst: ir::SetDst::Reg(ir::Reg("esp".into())),
                val: ir::Expr::BinOp {
                    kind: ir::BinOpKind::Add,
                    left: Rc::new(ir::Expr::Reg(ir::Reg("esp".into()))),
                    right: Rc::new(ir::Expr::Const(WORD_SIZE)),
                },
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

pub fn lift_and(operands: &[X86Operand], cs: &Capstone) -> ir::Block {
    if let [dst, src] = operands {
        let dst = &dst.op_type;
        let src = &src.op_type;
        ir::Block(vec![ir::Stmt::Set {
            dst: lift_set_dst(dst, cs),
            val: ir::Expr::BinOp {
                kind: ir::BinOpKind::And,
                left: Rc::new(lift_read_operand(dst, cs)),
                right: Rc::new(lift_read_operand(src, cs)),
            },
        }])
    } else {
        panic!("invalid amount of operands")
    }
}

pub fn lift_add(operands: &[X86Operand], cs: &Capstone) -> ir::Block {
    if let [dst, src] = operands {
        let dst = &dst.op_type;
        let src = &src.op_type;
        ir::Block(vec![ir::Stmt::Set {
            dst: lift_set_dst(dst, cs),
            val: ir::Expr::BinOp {
                kind: ir::BinOpKind::Add,
                left: Rc::new(lift_read_operand(dst, cs)),
                right: Rc::new(lift_read_operand(src, cs)),
            },
        }])
    } else {
        panic!("invalid amount of operands")
    }
}

pub fn lift_lea(operands: &[X86Operand], cs: &Capstone) -> ir::Block {
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
