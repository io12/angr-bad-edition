use capstone::prelude::*;

pub fn lift(bytes: &[u8], addr: u64) {
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode32)
        .syntax(arch::x86::ArchSyntax::Intel)
        .build()
        .expect("Failed to create Capstone object");
    let insns = cs.disasm_all(bytes, addr).expect("failed to disassemble");
    for ins in insns.iter() {
        println!("{}", ins);
    }
}
