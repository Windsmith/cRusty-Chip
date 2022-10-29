use crate::utils::get_bits;

pub struct Display {
    frame_pos: [[u8; 32]; 64],
}

impl Display {
    pub fn new() -> Self {
        Display {
            frame_pos: [[0; 32]; 64],
        }
    }

    pub fn flip_pixel(&mut self, x: usize, y: usize) -> bool {
        let mut collided = false;
        self.frame_pos[x][y] ^= 1;

        if self.frame_pos[x][y] != 1 {
            collided = true;
        }

        collided
    }

    pub fn draw_sprite(&mut self, sprite: &[u8], x: u8, y: u8) -> bool {
        let mut collided = false;
        for (y_index, byte) in sprite.iter().enumerate() {
            for (x_index, bit) in get_bits(*byte).iter().enumerate() {
                if *bit {
                    let x_pos = (x + x_index as u8) % 64;
                    let y_pos = (y + y_index as u8) % 32;
                    let collision_check =
                        &self.flip_pixel((x_pos as u8).into(), (y_pos as u8).into());

                    if collided == false && *collision_check == true {
                        collided = true;
                    }
                }
            }
        }
        collided
    }

    pub fn clear(&mut self) {
        self.frame_pos = [[0; 32]; 64];
    }

    pub fn get_frame(&mut self) -> [[u8; 32]; 64] {
        self.frame_pos
    }
}
