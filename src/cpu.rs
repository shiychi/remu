use crate::instruction::{self, Instruction};
use anyhow::Result;
use thiserror::Error;

#[derive(Default)]
pub struct Cpu {
    pub register: [u32; 32],
    pub pc: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fetch(&mut self, memory: &[u8]) -> u32 {
        let mut inst: u32 = 0;
        for i in 0..4 {
            let e: u32 = memory[self.pc + (3 - i)].into();
            inst = (inst << 8) + e;
        }
        self.pc += 4;
        inst
    }

    pub fn decode(&self, raw: u32) -> Result<Instruction> {
        instruction::parse(raw)
    }

    pub fn execute(&mut self, inst: Instruction) {
        match inst {
            Instruction::Add(i) => instruction::add(self, i),
        }
    }
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Unexpected opcode: 0x{0:x}")]
    OpcodeError(u8),
    #[error("Unexpected funct3: {0}")]
    Funct3Error(u8),
    #[error("Unexpected funct7: {0}")]
    Funct7Error(u8),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::*;

    #[test]
    fn test_add_instruction() {
        let mut cpu = Cpu::new();

        let inst = cpu.decode(0x01d28fb3).unwrap();
        assert_eq!(
            inst,
            Instruction::Add(RTypeInstruction {
                opcode: 0x33,
                funct3: 0,
                funct7: 0,
                rd: 31,
                rs1: 5,
                rs2: 29
            })
        );

        cpu.register[5] = 4; // rs1
        cpu.register[29] = 2; // rs2
        cpu.execute(inst);
        assert_eq!(
            cpu.register,
            [
                0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 2, 0, 6,
            ]
        );
    }
}
