use crate::instruction::{self, Instruction};

#[derive(Default)]
pub struct Cpu {
    pub register: [u32; 32],
    pub pc: usize,
}

impl Cpu {
    pub fn fetch(&mut self, memory: &Vec<u8>) -> u32 {
        let mut inst: u32 = 0;
        for i in 0..4 {
            let e: u32 = memory[self.pc + (3 - i)].into();
            inst = (inst << 8) + e;
        }
        self.pc += 4;
        inst
    }

    pub fn decode(self, raw: u32) -> Result<Instruction, String> {
        instruction::parse(raw)
    }
}