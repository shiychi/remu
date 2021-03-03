use remu::cpu::Cpu;
use remu::emulator::Emulator;
use std::fs::File;
use std::io::Read;

fn main() {
    let binary_path = std::env::args().nth(1).expect("must be set binary path");
    let mut binary_file = File::open(binary_path).expect("failed to open file");
    let mut binary = Vec::new();
    binary_file
        .read_to_end(&mut binary)
        .expect("failed to read file");
    let cpu = Cpu::default();
    let mut emulator = Emulator::default();
    emulator.write_binary(binary);
}
