use std::{thread::sleep, time::Duration};

use rand::Rng;
use sdl2::{event::Event, keyboard::Scancode, Sdl};

use crate::{memory::Memory, display::Display, keypad::Keypad, opcodes};

pub struct Cpu {
    pub vx: [u8; 16],
    pub i: u16,
    pub delay: u8,
    pub sound: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
}

impl Cpu {
    pub fn new() -> Self{
        Cpu {
            vx: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
            pc: 512,
            sp: 0,
            stack: [0; 16]
        }
    }

    pub fn run_program(&mut self, memory: &mut Memory, display: &mut Display, sdl_context: &Sdl) {
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut keypad = Keypad::new();

        loop {
            let hi = memory.read(self.pc.into()); // first byte of instruction
            let lo = memory.read((self.pc + 1).into()); // second byte of instruction (nn)
            let instruction: u16 = (((hi as u16) << 8) | lo as u16).into();

            let k = hi >> 4; // first nibble of instruction
            let x = hi & 0x0F; // second nibble of instruction
            let y = lo >> 4; // third nibble of instruction
            let n = lo & 0x0F; // fourth nibble of instruction
            let nnn = (instruction << 4) >> 4; // second-third-fourth nibbles
        
            loop {
                match event_pump.poll_event() {
                    Some(Event::KeyDown {timestamp, window_id, keycode, scancode, keymod, repeat }) => {
                        match scancode {
                            Some(Scancode::Num1) => {keypad.key_down(1)}
                            Some(Scancode::Num2) => {keypad.key_down(2)}
                            Some(Scancode::Num3) => {keypad.key_down(3)}
                            Some(Scancode::Num4) => {keypad.key_down(0xC)}
                            Some(Scancode::Q) => {keypad.key_down(4)}
                            Some(Scancode::W) => {keypad.key_down(5)}
                            Some(Scancode::E) => {keypad.key_down(6)}
                            Some(Scancode::R) => {keypad.key_down(0xD)}
                            Some(Scancode::A) => {keypad.key_down(7)}
                            Some(Scancode::S) => {keypad.key_down(8)}
                            Some(Scancode::D) => {keypad.key_down(9)}
                            Some(Scancode::F) => {keypad.key_down(0xE)}
                            Some(Scancode::Z) => {keypad.key_down(0xA)}
                            Some(Scancode::X) => {keypad.key_down(0)}
                            Some(Scancode::C) => {keypad.key_down(0xB)}
                            Some(Scancode::V) => {keypad.key_down(0xF)}
                            _ => {}
                        }
                    }
                    None => break,
                    _ => {}
                }
            }

            match k {
                0 => opcodes::opcode0(self, display, n),
                1 => opcodes::opcode1(self, nnn),
                2 => opcodes::opcode2(self, nnn),
                3 => opcodes::opcode3(self, x, lo),
                4 => opcodes::opcode4(self, x, lo),
                5 => opcodes::opcode5(self, x, y),
                6 => opcodes::opcode6(self, x, lo),
                7 => opcodes::opcode7(self, x, lo),
                8 => opcodes::opcode8(self, n, x, y),
                9 => opcodes::opcode9(self, x, y),
                0xA => opcodes::opcodeA(self, nnn),
                0xB => opcodes::opcodeB(self, nnn),
                0xC => opcodes::opcodeC(self, x, lo),
                0xD => opcodes::opcodeD(self, display, memory, n, x, y),
                0xE => opcodes::opcodeE(self, &mut keypad, n, x),
                0xF => opcodes::opcodeF(self, memory, &mut event_pump, x, lo),
                _ => ()
            }

            //sleep(Duration::from_secs_f32(1.0));
            sleep(Duration::from_secs_f32(0.00142857)); // 700 instructions per second

            let mut delay_counter = 0;
            delay_counter += 1; 
            if delay_counter == 12 {
                self.delay -= 1;
                delay_counter = 0;
            }

            keypad.reset() 
        }
        
    }
}