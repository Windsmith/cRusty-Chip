pub fn get_bits(byte: u8) -> [bool; 8] {
    let mut bits = [false; 8];
    for i in 0..8 {
        bits[i] = (byte << i) >> 7 == 1;
    }
    bits
}