fn main() {
    let path = "/bin/true";

    let prog = angr_bad::lift::x86::lift_elf_path(path);
    println!("{}", prog);

    let _mem = {
        let mut mem = angr_bad::mem::Mem::default();
        mem.load_elf_path(path);
        mem
    };
}
