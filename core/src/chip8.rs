use std::fs;

use crate::{cpu::Cpu, display::Display, memory::Memory};

pub struct Chip8 {
    memory: Memory,
    display: Display,
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: Memory::new(),
            display: Display::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_program(&mut self, game: &str) {
        let data: Vec<u8> = fs::read(format!("../programs/{}.ch8", game)).unwrap();

        self.memory.load_program(data);
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.memory, &mut self.display)
    }

    pub fn tick_timers(&mut self) {
        self.cpu.tick_timers()
    }

    pub fn key_down(&mut self, key: usize) {
        self.cpu.keys[key] = true
    }

    pub fn key_up(&mut self, key: usize) {
        self.cpu.keys[key] = false
    }

    pub fn get_display(&mut self) -> [[u8; 32]; 64] {
        self.display.get_frame()
    }
}
