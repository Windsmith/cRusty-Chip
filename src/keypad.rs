use std::process;

use sdl2::{event::Event, keyboard::Scancode, EventPump};

pub struct Keypad {
    keys: [bool; 16] // 0 to F
}

impl Keypad {
    pub fn new() -> Self {
        Keypad { keys: [false;16] }
    }

    pub fn reset(&mut self) {
        self.keys = [false;16];
    }

    pub fn key_down(&mut self, key: usize) {
        self.keys[key] = true
    }

    pub fn is_key_down(&mut self, key:usize) -> bool {
        if self.keys[key] {return true}
        false
    }

    pub fn is_key_up(&mut self, key:usize) -> bool {
        if !self.keys[key] {return true}
        false
    }

    pub fn register_key_downs(&mut self, event_pump: &mut EventPump) {
        loop {
            match event_pump.poll_event() {
                Some(Event::KeyDown {timestamp, window_id, keycode, scancode, keymod, repeat }) => {
                    match scancode {
                        Some(Scancode::Num1) => {self.key_down(1)}
                        Some(Scancode::Num2) => {self.key_down(2)}
                        Some(Scancode::Num3) => {self.key_down(3)}
                        Some(Scancode::Num4) => {self.key_down(0xC)}
                        Some(Scancode::Q) => {self.key_down(4)}
                        Some(Scancode::W) => {self.key_down(5)}
                        Some(Scancode::E) => {self.key_down(6)}
                        Some(Scancode::R) => {self.key_down(0xD)}
                        Some(Scancode::A) => {self.key_down(7)}
                        Some(Scancode::S) => {self.key_down(8)}
                        Some(Scancode::D) => {self.key_down(9)}
                        Some(Scancode::F) => {self.key_down(0xE)}
                        Some(Scancode::Z) => {self.key_down(0xA)}
                        Some(Scancode::X) => {self.key_down(0)}
                        Some(Scancode::C) => {self.key_down(0xB)}
                        Some(Scancode::V) => {self.key_down(0xF)}
                        //Quit program
                        Some(Scancode::Escape) => {process::exit(0)}
                        _ => {}
                    }
                }
                None => break,
                _ => {}
            }
        }
    }
}