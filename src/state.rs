use crate::mem::Mem;

use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    pub mem: Mem,
    pub regs: HashMap<String, i64>,
}
