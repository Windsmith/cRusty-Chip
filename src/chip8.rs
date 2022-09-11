use std::fs;

use sdl2::{Sdl, EventPump};

use crate::{display::Display, memory::Memory, cpu::Cpu};

pub struct Chip8 {
    event_pump: EventPump,
    memory: Memory,
    display: Display,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Self {
        
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        Chip8 {
            event_pump: event_pump,
            memory: Memory::new(),
            display: Display::new(&sdl_context),
            cpu: Cpu::new()
        }
    }

    pub fn run(&mut self) {
        let data: Vec<u8> = fs::read("./programs/glitchGhost.ch8").unwrap();
    
        self.memory.load_program(data);
        self.cpu.run_program(&mut self.memory, &mut self.display, &mut self.event_pump)
    }
}