mod memory;
mod display;
mod cpu;
mod utils;

use std::fs;
use std::path::Path;

use bitvec::vec::BitVec;
use bitvec::{bitvec, bits};
use bitvec::prelude::Lsb0;

use cpu::Cpu;
use display::Display;
use memory::Memory;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use utils::get_bits;

fn main() {
    let data: Vec<u8> = fs::read("programs/IBM Logo.ch8").unwrap();
    
    let mut chip8_memory = Memory::new();
    chip8_memory.load_program(data);

    let mut chip8_display = Display::new();

    let mut chip8_cpu = Cpu::new();

    chip8_cpu.run_program(&mut chip8_memory, &mut chip8_display);
    println!("DONE");
    chip8_display.render();
    loop {}
}
