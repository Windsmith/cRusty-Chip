#[derive(Debug)]
pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory { ram: [0; 4096] };

        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];

        let mut index = 0;
        for sprite in sprites {
            for byte in sprite {
                memory.ram[index] = byte;
                index += 1;
            }
        }

        memory
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        for (index, byte) in program.iter().enumerate() {
            self.ram[index + 512] = *byte;
        }
    }

    pub fn read(&mut self, index: usize) -> u8 {
        self.ram[index]
    }

    pub fn write_to_ram(&mut self, index: usize, data: u8) {
        self.ram[index] = data;
    }

    pub fn reset(&mut self) {
        self.ram = [0; 4096];
    }
}
