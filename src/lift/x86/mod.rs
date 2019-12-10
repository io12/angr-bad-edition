mod ins;

use crate::ir;

use capstone::arch::x86::X86Insn;
use capstone::arch::x86::X86Operand;
use capstone::prelude::*;
use capstone::Insn;
use goblin::elf::Elf;

pub fn opcode(ins: &Insn) -> X86Insn {
    unsafe { std::mem::transmute(ins.id().0) }
}

pub fn ins_operands<'a>(ins: &'a Insn, cs: &Capstone) -> Vec<X86Operand> {
    if let ArchDetail::X86Detail(detail) = cs
        .insn_detail(ins)
        .expect("failed getting instruction details")
        .arch_detail()
    {
        detail.operands().collect()
    } else {
        panic!("instruction detail is not x86")
    }
}

/// Lift instruction to IR
pub fn lift_ins(ins: &Insn, cs: &Capstone) -> ir::Block {
    let operands = ins_operands(ins, cs);

    match opcode(ins) {
        X86Insn::X86_INS_XOR => ins::lift_xor(&operands, cs),
        X86Insn::X86_INS_PUSH => ins::lift_push(&operands, cs),
        X86Insn::X86_INS_MOV => ins::lift_mov(&operands, cs),
        _ => ir::Block(vec![ir::Stmt::Asm(ins.bytes().to_vec())]),
    }
}

/// Lift a function from bytes
pub fn lift_func(bytes: &[u8], addr: u64) -> ir::Function {
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode32)
        .syntax(arch::x86::ArchSyntax::Intel)
        .detail(true)
        .build()
        .expect("Failed to create Capstone object");
    let insns = cs.disasm_all(bytes, addr).expect("failed to disassemble");
    let mut block = ir::Block(vec![]);
    for block_app in insns.iter().map(|ins| lift_ins(&ins, &cs)) {
        block.0.append(&mut block_app.0.clone());
    }
    let mut func = ir::Function::new();
    func.cfg.add_node(block);
    func
}

/// Lift an ELF function
fn lift_elf_func(elf: &Elf, bytes: &[u8], addr: u64) -> ir::Function {
    let sh = elf
        .section_headers
        .iter()
        .find(|sh| {
            let mem_range = sh.sh_addr..(sh.sh_addr + sh.sh_size);
            mem_range.contains(&addr)
        })
        .expect("elf has no section header at address");

    let addr_sz = addr as usize;
    let sh_off = sh.sh_offset as usize;
    let sh_size = sh.sh_size as usize;
    let sh_addr = sh.sh_addr as usize;

    // Offset of function in section
    let func_off = addr_sz - sh_addr;

    let file_range = sh_off..(sh_off + sh_size);
    let bytes = &bytes[file_range];
    let bytes = &bytes[func_off..];
    lift_func(bytes, addr)
}

/// Lift ELF to IR program
pub fn lift_elf(elf: &Elf, bytes: &[u8]) -> ir::Program {
    ir::Program {
        funcs: btreemap! { "_start".to_string() => lift_elf_func(elf, bytes, elf.entry) },
    }
}
