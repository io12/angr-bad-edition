use crate::ir;

use std::collections::HashMap;
use std::iter;
use std::ops::Range;

use goblin::elf::program_header::ProgramHeader as ElfProgramHeader;
use goblin::elf::program_header::PT_LOAD;
use goblin::elf::Elf;

const PAGE_SIZE: usize = 0x1000;

#[derive(Debug, Default)]
pub struct Mem {
    pages: HashMap<i64, Page>,
}

impl Mem {
    pub fn load_elf(&mut self, elf_bytes: &[u8], elf: &Elf) {
        elf.program_headers
            .iter()
            .filter(|phdr| phdr.p_type == PT_LOAD)
            .for_each(|phdr| self.load_elf_phdr(elf_bytes, phdr))
    }

    fn load_elf_phdr(&mut self, elf_bytes: &[u8], phdr: &ElfProgramHeader) {
        let bytes = &elf_bytes[phdr.file_range()];
        let mem_range = phdr.vm_range();

        self.load_bytes(bytes, mem_range)
    }

    fn load_bytes(&mut self, bytes: &[u8], mem_range: Range<usize>) {
        for (i, vaddr) in mem_range.enumerate() {
            self.set_byte(vaddr, *bytes.get(i).unwrap_or(&0))
        }
    }

    fn set_byte(&mut self, vaddr: usize, byte: u8) {
        let ind = Mem::page_index(vaddr) as i64;
        let off = Mem::page_offset(vaddr);
        self.pages.entry(ind).or_default().bytes[off] = ir::Expr::Const(byte as i64);
    }

    fn page_index(vaddr: usize) -> usize {
        vaddr / PAGE_SIZE
    }

    /// Return offset into page
    fn page_offset(vaddr: usize) -> usize {
        vaddr % PAGE_SIZE
    }
}

#[derive(Debug)]
struct Page {
    // TODO: Change this to array once const generics are stable
    bytes: Vec<ir::Expr>,
}

impl Default for Page {
    fn default() -> Page {
        Page {
            bytes: iter::repeat(ir::Expr::Const(0)).take(PAGE_SIZE).collect(),
        }
    }
}
