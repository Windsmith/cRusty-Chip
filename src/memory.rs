pub struct Memory {
    ram: [u8; 4096]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {ram: [0; 4096] }
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        for (index, byte) in program.iter().enumerate() {
            self.ram[index + 512] = *byte;
        }
    }
}