mod memory;

use std::fs;
use std::path::Path;

use memory::Memory;

fn main() {
    let data: Vec<u8> = fs::read("programs/RPS.ch8").unwrap();
    
    let mut chip8_memory = Memory::new();
    chip8_memory.load_program(data);

    println!("{:x?}", chip8_memory);
}
