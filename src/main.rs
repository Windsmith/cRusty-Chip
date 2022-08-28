mod memory;
mod display;

use std::fs;
use std::path::Path;

use display::Display;
use memory::Memory;
use sdl2::pixels::Color;
use sdl2::rect::Point;

fn main() {
    let data: Vec<u8> = fs::read("programs/RPS.ch8").unwrap();
    
    let mut chip8_memory = Memory::new();
    chip8_memory.load_program(data);

    let mut chip8_display = Display::new();

    loop {}
}
