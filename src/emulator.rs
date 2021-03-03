#[derive(Default)]
pub struct Emulator {
    pub memory: Vec<u8>,
}

impl Emulator {
    pub fn write_binary(&mut self, binary_str: Vec<u8>) {
        self.memory = binary_str;
    }
}
