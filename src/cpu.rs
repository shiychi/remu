#[derive(Default)]
pub struct Cpu {
    pub register: [u32; 32],
    pub pc: usize,
}
