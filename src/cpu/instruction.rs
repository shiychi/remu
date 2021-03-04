use crate::cpu::Cpu;
use std::convert::TryInto;

pub fn parse(raw: u32) -> Result<Instruction, String> {
    let opcode = raw & 0b1111111;

    match opcode {
        0x33 => {
            let inst = RTypeInstruction::default(raw);
            inst.parse()
        }
        _ => Err("Unexpected opcode".into()),
    }
}

#[derive(Debug)]
pub enum Instruction {
    Add(RTypeInstruction),
}

#[derive(Debug)]
pub struct RTypeInstruction {
    pub opcode: u8,
    pub funct3: u8,
    pub funct7: u8,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}

#[derive(Debug)]
pub struct _ITypeInstruction {
    pub opcode: u8,
    pub funct3: u8,
    pub rd: u8,
    pub rs1: u8,
    pub imm: u32,
}

#[derive(Debug)]
pub struct _STypeInstruction {
    pub opcode: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: u32,
}

#[derive(Debug)]
pub struct _UTypeInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub imm: u32,
}

impl RTypeInstruction {
    pub fn default(raw: u32) -> Self {
        let rd: u8 = ((raw >> 7) & 0b11111).try_into().unwrap();
        let funct3: u8 = ((raw >> 12) & 0b111).try_into().unwrap();
        let rs1: u8 = ((raw >> 15) & 0b11111).try_into().unwrap();
        let rs2: u8 = ((raw >> 20) & 0b11111).try_into().unwrap();
        let funct7: u8 = (raw >> 25).try_into().unwrap();

        Self {
            opcode: 0x33,
            funct3,
            funct7,
            rd,
            rs1,
            rs2,
        }
    }

    // TODO: add error type
    pub fn parse(self) -> Result<Instruction, String> {
        match self.funct3 {
            0 => match self.funct7 {
                0 => Ok(Instruction::Add(self)),
                _ => Err("Unexpected funct7".into()),
            },
            _ => Err("Unexpected funct3".into()),
        }
    }
}

pub fn add(cpu: &mut Cpu, i: RTypeInstruction) {
    let rs1 = cpu.register[i.rs1 as usize];
    let rs2 = cpu.register[i.rs2 as usize];
    cpu.register[i.rd as usize] = rs1 + rs2;
}
