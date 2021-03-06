use crate::cpu::Cpu;
use anyhow::Result;

#[derive(Default)]
pub struct Emulator {
    cpu: Cpu,
    pub memory: Vec<u8>,
}

impl Emulator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_binary(&mut self, binary_v: Vec<u8>) {
        self.memory = binary_v;
    }

    pub fn start(&mut self) -> Result<()> {
        while self.cpu.pc < self.memory.len() {
            println!("cpu: {:?}", self.cpu);
            let raw_inst = self.cpu.fetch(&self.memory);
            println!("raw_inst: 0x{:x?}", raw_inst);
            let inst = self.cpu.decode(raw_inst)?;
            println!("inst: {:?}", inst);
            self.cpu.execute(inst);
            println!("execute instruction!");
        }
        println!("cpu: {:?}", self.cpu);
        Ok(())
    }
}
