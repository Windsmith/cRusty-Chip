use crate::{display::Display, memory::Memory, opcodes};

pub struct Cpu {
    pub vx: [u8; 16],
    pub i: u16,
    pub delay: u8,
    pub sound: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub keys: [bool; 16],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            vx: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
            pc: 512,
            sp: 0,
            stack: [0; 16],
            keys: [false; 16],
        }
    }

    pub fn tick(&mut self, memory: &mut Memory, display: &mut Display) {
        let hi = memory.read(self.pc.into()); // first byte of instruction
        let lo = memory.read((self.pc + 1).into()); // second byte of instruction (nn)
        let instruction: u16 = (((hi as u16) << 8) | lo as u16).into();

        let k = hi >> 4; // first nibble of instruction
        let x = hi & 0x0F; // second nibble of instruction
        let y = lo >> 4; // third nibble of instruction
        let n = lo & 0x0F; // fourth nibble of instruction
        let nnn = (instruction << 4) >> 4; // second-third-fourth nibbles

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
            0xE => opcodes::opcodeE(self, n, x),
            0xF => opcodes::opcodeF(self, memory, x, lo),
            _ => (),
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay > 0 {
            self.delay -= 1
        }
    }
}
