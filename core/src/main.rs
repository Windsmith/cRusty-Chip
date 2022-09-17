mod memory;
mod display;
mod cpu;
mod utils;
mod keypad;
mod opcodes;
mod chip8;

use std::fs;
use std::path::Path;

use chip8::Chip8;
use cpu::Cpu;
use display::Display;
use memory::Memory;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use utils::get_bits;

fn main() {
    let mut chip8_machine = Chip8::new();

    chip8_machine.run();
}
