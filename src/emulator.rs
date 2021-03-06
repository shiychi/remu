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
        // TODO: add debug info
        while self.cpu.pc < self.memory.len() {
            let raw_inst = self.cpu.fetch(&self.memory);
            let inst = self.cpu.decode(raw_inst)?;
            self.cpu.execute(inst);
        }
        Ok(())
    }
}
