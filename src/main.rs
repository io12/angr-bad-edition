static SHELLCODE: &[u8] =
    b"\x31\xc0\x50\x68\x2f\x2f\x73\x68\x68\x2f\x62\x69\x6e\x89\xe3\x50\x53\x89\xe1\xb0\x0b\xcd\x80";

fn main() {
    let expr = angr_bad::lift::x86::lift_func(SHELLCODE, 0);
    println!("{:#?}", expr);

    let _mem = {
        let mut mem = angr_bad::mem::Mem::default();
        mem.load_elf_path("/bin/true");
        mem
    };
}
