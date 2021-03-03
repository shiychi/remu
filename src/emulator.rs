pub struct Emulator {
    pub memory: Vec<u8>,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator { memory: Vec::new() }
    }

    pub fn write_binary(&mut self, binary_str: Vec<u8>) {
        self.memory = binary_str;
    }
}
