use crate::{memory::Memory, display::Display};

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

    pub fn run_program(&mut self, memory: &mut Memory, display: &mut Display) {
        loop {
            let hi = memory.read(self.pc.into()); // first byte of instruction
            let lo = memory.read((self.pc + 1).into()); // second byte of instruction (nn)
            let instruction: u16 = (((hi as u16) << 8) | lo as u16).into();

            let k = hi >> 4; // first nibble of instruction
            let x = hi & 0x0F; // second nibble of instruction
            let y = lo >> 4; // third nibble of instruction
            let n = lo & 0x0F; // fourth nibble of instruction
            let nnn = (instruction << 4) >> 4; // second-third-fourth nibbles
        
            match k {
                // clear screen
                0 => {
                    println!("0");
                    display.clear();
                    self.pc += 2;
                }
                // Jump to location nnn
                19 => {
                    self.pc = nnn;
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
                    self.vx[x as usize] = lo + self.vx[x as usize];
                    self.pc += 2;
                }
                // Set I = nnn.
                0xA => {
                    println!("A");
                    self.i = nnn;
                    self.pc += 2;
                }
                //Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                0xD => {
                    println!("D");
                    let x_pos = self.vx[x as usize];
                    let y_pos = self.vx[y as usize];
                    let mut sprite: Vec<u8> = Vec::new();
                    for index in 0..n {
                        sprite.push(memory.read((self.i + index as u16).into()))
                    }
                    let collision = display.draw_sprite(&sprite, x_pos, y_pos);

                    if collision {self.vx[0xF] = 1}
                    else {self.vx[0xF] = 1}

                    self.pc += 2;
                }
                _ => break // CHANGE LATER
            }
        }
    }
}