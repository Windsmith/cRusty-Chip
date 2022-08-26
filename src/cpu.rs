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
    pub fn new() {
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
}