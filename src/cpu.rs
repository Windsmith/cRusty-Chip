use std::{thread::sleep, time::Duration};

use rand::Rng;
use sdl2::{event::Event, keyboard::Scancode, Sdl};

use crate::{memory::Memory, display::Display, keypad::Keypad};

pub struct Cpu {
    vx: [u8; 16],
    i: u16,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
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
                
                0 => {
                    match n {
                        // clear screen
                        0 => {
                            println!("00");
                            display.clear();
                            self.pc += 2;
                        }
                        // Return from a subroutine.
                        0xE => {
                            println!("0E");
                            self.sp -= 1;
                            self.pc = self.stack[self.sp as usize];
                            self.pc += 2;
                        }
                        _ => ()
                    }
                    
                }
                // Jump to location nnn
                1 => {
                    println!("1");
                    self.pc = nnn;
                }
                // Call subroutine at nnn.
                2 => {
                    println!("2");
                    self.stack[self.sp as usize] = self.pc;
                    self.sp += 1;
                    self.pc = nnn;
                }
                // Skip next instruction if Vx = kk.
                3 => {
                    println!("3");
                    if self.vx[x as usize] == lo { self.pc += 2; }
                    self.pc += 2;
                }
                // Skip next instruction if Vx != kk.
                4 => {
                    println!("4");
                    if self.vx[x as usize] != lo { self.pc += 2; }
                    self.pc += 2;
                }
                // Skip next instruction if Vx = Vy.
                5 => {
                    println!("5");
                    if self.vx[x as usize] == self.vx[y as usize] { self.pc += 2; }
                    self.pc += 2;
                }
                // Set Vx = kk.
                6 => {
                    println!("6");
                    self.vx[x as usize] = lo;
                    self.pc += 2;
                }
                // Set Vx = Vx + kk.
                7 => {
                    println!("7");
                    self.vx[x as usize] = lo.wrapping_add(self.vx[x as usize]);
                    self.pc += 2;
                }
                8 => {
                    match n {
                        // Set Vx = Vy.
                        0 => {
                            println!("80");
                            self.vx[x as usize] = self.vx[y as usize];
                            self.pc += 2;
                        }
                        // Set Vx = Vx OR Vy.
                        1 => {
                            println!("81");
                            self.vx[x as usize] = self.vx[x as usize] | self.vx[y as usize];
                            self.pc += 2;
                        }
                        // Set Vx = Vx AND Vy.
                        2 => {
                            println!("82");
                            self.vx[x as usize] = self.vx[x as usize] & self.vx[y as usize];
                            self.pc += 2;
                        }
                        // Set Vx = Vx XOR Vy.
                        3 => {
                            println!("83");
                            self.vx[x as usize] = self.vx[x as usize] ^ self.vx[y as usize];
                            self.pc += 2;
                        }
                        // Set Vx = Vx + Vy, set VF = carry.
                        4 => {
                            println!("84");
                            let one = self.vx[x as usize];
                            let two = self.vx[y as usize];

                            if (one as u16 + two as u16) > u8::MAX as u16 {self.vx[0xF] = 1}
                            else {self.vx[0xF] = 0}

                            self.vx[x as usize] = one.wrapping_add(two); // MAYBE NOT WRAPPING ADD
                            self.pc += 2
                        }
                        // Set Vx = Vx - Vy, set VF = NOT borrow.
                        5 => {
                            println!("85");
                            let one = self.vx[x as usize];
                            let two = self.vx[y as usize];

                            if one > two {self.vx[0xF] = 1}
                            else {self.vx[0xF] = 0}

                            self.vx[x as usize] = one.wrapping_sub(two); // MAYBE NOT WRAPPING SUB
                            self.pc += 2;
                        }
                        // Set Vx = Vx SHR 1.
                        6 => {
                            println!("86");
                            let one = self.vx[x as usize];

                            if (one << 7) >> 7 == 1 {self.vx[0xF] = 1} 
                            else {self.vx[0xF] = 0}

                            self.vx[x as usize] = self.vx[x as usize] >> 1;
                            self.pc += 2
                        }
                        // Set Vx = Vy - Vx, set VF = NOT borrow.
                        7 => {
                            println!("87");
                            let one = self.vx[x as usize];
                            let two = self.vx[y as usize];

                            if two > one {self.vx[0xF] = 1}
                            else {self.vx[0xF] = 0}

                            self.vx[x as usize] = two.wrapping_sub(one); // MAYBE NOT WRAPPING SUB
                            self.pc += 2
                        }
                        // Set Vx = Vx SHL 1.
                        0xE => {
                            println!("8E");
                            let one = self.vx[x as usize];

                            if one >> 7 == 1 {self.vx[0xF] = 1} 
                            else {self.vx[0xF] = 0}

                            self.vx[x as usize] = self.vx[x as usize] << 1;
                            self.pc += 2
                        }
                        _ => ()
                    }
                    
                }
                // Skip next instruction if Vx != Vy.
                9 => {
                    println!("9");
                    if self.vx[x as usize] != self.vx[y as usize] { self.pc += 2; }
                    self.pc += 2;
                }
                // Set I = nnn.
                0xA => {
                    println!("A");
                    self.i = nnn;
                    self.pc += 2;
                }
                // Jump to location nnn + V0.
                0xB => {
                    println!("B");
                    self.pc = self.vx[0] as u16 + nnn;
                }
                // Set Vx = random byte AND kk.
                0xC => {
                    println!("C");
                    let random_num: u8 = rand::thread_rng().gen();
                    self.vx[x as usize] = random_num & lo;
                    self.pc += 2;
                }
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                0xD => {
                    println!("D");
                    let x_pos = self.vx[x as usize] % 64;
                    let y_pos = self.vx[y as usize] % 32;
                    let mut sprite: Vec<u8> = Vec::new();
                    for index in 0..n {
                        sprite.push(memory.read((self.i + index as u16).into()))
                    }
                    let collision = display.draw_sprite(&sprite, x_pos, y_pos);

                    if collision {self.vx[0xF] = 1}
                    else {self.vx[0xF] = 0}

                    self.pc += 2;
                    display.render();
                }
                0xE => {
                    match n {
                        // Skip next instruction if key with the value of Vx is pressed.
                        0xE => {
                            println!("EE");
                            if keypad.is_key_down(self.vx[x as usize] as usize) {self.pc += 2}
                            self.pc += 2;
                        }
                        // Skip next instruction if key with the value of Vx is not pressed.
                        0x1 => {
                            println!("E1");
                            if keypad.is_key_up(self.vx[x as usize] as usize) {self.pc += 2}
                            self.pc += 2;
                        }
                        _ => {}
                    }
                }
                0xF => {
                    match lo {
                        // Set Vx = delay timer value.
                        0x07 => {
                            println!("F07");
                            self.vx[x as usize] = self.delay;
                            self.pc += 2;
                        }
                        // Wait for a key press, store the value of the key in Vx.
                        0x0A => {
                            println!("F0A");
                            let mut key_pressed = false;
                            let mut key: u8 = 0;
                            while !key_pressed {
                                match event_pump.wait_event() {
                                    Event::KeyDown {timestamp, window_id, keycode, scancode, keymod, repeat } => {
                                        key_pressed = true;
                                        key = match scancode {
                                            Some(Scancode::Num1) => 1,
                                            Some(Scancode::Num2) => 2,
                                            Some(Scancode::Num3) => 3,
                                            Some(Scancode::Num4) => 0xC,
                                            Some(Scancode::Q) => 4,
                                            Some(Scancode::W) => 5,
                                            Some(Scancode::E) => 6,
                                            Some(Scancode::R) => 0xD, 
                                            Some(Scancode::A) => 7,
                                            Some(Scancode::S) => 8,
                                            Some(Scancode::D) => 9,
                                            Some(Scancode::F) => 0xE,
                                            Some(Scancode::Z) => 0xA,
                                            Some(Scancode::X) => 0,
                                            Some(Scancode::C) => 0xB,
                                            Some(Scancode::V) => 0xF,
                                            _ => { 
                                                key_pressed = false;
                                                0
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            self.vx[x as usize] = key;
                            self.pc += 2;
                        }
                        // Set delay timer = Vx.
                        0x15 => {
                            println!("F15");
                            self.delay = self.vx[x as usize];
                            self.pc += 2;
                        }
                        // Set sound timer = Vx.
                        0x18 => {
                            println!("F18");
                            self.sound = self.vx[x as usize];
                            self.pc += 2;
                        }
                        // Set I = I + Vx.
                        0x1E => {
                            println!("F1E");
                            self.i = self.i + self.vx[x as usize] as u16;
                            self.pc += 2;
                        }
                        // Set I = location of sprite for digit Vx.
                        0x29 => {
                            println!("F29");
                            self.i = (self.vx[x as usize] * 5) as u16;
                            self.pc += 2
                        }
                        // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                        0x33 => {
                            println!("F33");
                            let num = self.vx[x as usize];
                            println!("{} {} {} {}", num, num / 100, (num % 100) / 10, (num % 100) % 10);
                            memory.write_to_ram(self.i.into(), num / 100);
                            memory.write_to_ram((self.i + 1).into(), (num % 100) / 10);
                            memory.write_to_ram((self.i + 2).into(), (num % 100) % 10);
                            self.pc += 2
                        }
                        // Store registers V0 through Vx in memory starting at location I.
                        0x55 => {
                            println!("F55");
                            for index in 0..(x + 1) {
                                memory.write_to_ram((self.i + index as u16).into(), self.vx[index as usize])
                            }
                            self.pc += 2;
                        }
                        // Read registers V0 through Vx from memory starting at location I.
                        0x65 => {
                            println!("F65");
                            for index in 0..(x + 1) {
                                self.vx[index as usize] = memory.read((self.i + index as u16).into());
                            }
                            self.pc += 2;
                        }
                        _ => {}
                    }
                }
                _ => ()
            }

            //sleep(Duration::from_secs_f32(1.0));
            sleep(Duration::from_secs_f32(0.00142857)); // 700 instructions per second
            keypad.reset() 
        }
        
    }
}