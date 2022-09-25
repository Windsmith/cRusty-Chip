use core::chip8::Chip8;

fn main() {
    let mut chip8: Chip8 = Chip8::new();
    chip8.load_program("abc");
}
