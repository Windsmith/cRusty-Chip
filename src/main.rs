mod memory;
mod display;
mod cpu;
mod utils;
mod keypad;

use std::fs;
use std::path::Path;

use cpu::Cpu;
use display::Display;
use memory::Memory;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use utils::get_bits;

fn main() {
    let data: Vec<u8> = fs::read("./programs/RPS.ch8").unwrap();
    
    let sdl_context = sdl2::init().unwrap();

    let mut chip8_memory = Memory::new();
    chip8_memory.load_program(data);

    let mut chip8_display = Display::new(&sdl_context);

    let mut chip8_cpu = Cpu::new();

    chip8_cpu.run_program(&mut chip8_memory, &mut chip8_display, &sdl_context);
}
