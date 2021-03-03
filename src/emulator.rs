pub struct Emulator {
    pub memory: Vec<u8>,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator { memory: Vec::new() }
    }
}
