use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Point, VideoSubsystem, Sdl};

use crate::utils::get_bits;

pub struct Display {
    frame_pos: [[u8;32];64],
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Self {

        Display {
            frame_pos: [[0; 32];64],
        }
    }

    pub fn flip_pixel(&mut self, mut x: usize, mut y: usize) -> bool
    {
        let mut collided = false;
        self.frame_pos[x][y] ^= 1;
    
        if self.frame_pos[x][y] != 1 {
            collided = true;
        }
        
        collided
    }

    pub fn draw_sprite(&mut self, sprite: &[u8], x: u8, y: u8) -> bool{
        let mut collided = false;
        for (y_index, byte) in sprite.iter().enumerate() {
            for (x_index, bit) in get_bits(*byte).iter().enumerate() {
                if *bit {
                    let collision_check = &self.flip_pixel((x + x_index as u8).into(), (y + y_index as u8).into());

                    if collided == false && *collision_check == true {collided = true;}
                }
            }
        }
        collided
    }

    pub fn clear(&mut self) {
        self.frame_pos = [[0; 32];64];
    }
}