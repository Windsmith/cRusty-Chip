use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Point, VideoSubsystem, Sdl};

use crate::utils::get_bits;

pub struct Display {
    width: u8,
    height: u8,
    canvas: Canvas<Window>,
    frame_pos: [[u8;32];64],
    scale: usize,
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Self {
        let scale: usize = 15;

        let video = sdl_context.video().unwrap();
        let window = video.window("test", 64*scale as u32, 32*scale as u32).position_centered().build().unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
    
        canvas.set_scale(15.0, 15.0);

        Display {
            width: 64,
            height: 32,
            canvas: canvas,
            frame_pos: [[0; 32];64],
            scale: scale,
        }
    }

    pub fn flip_pixel(&mut self, mut x: usize, mut y: usize) -> bool
    {
        let mut collided = false;
        self.frame_pos[x][y] ^= 1;
    
        if self.frame_pos[x][y] == 1 {
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        }
        else {
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            collided = true;
        }

        self.canvas.draw_point(Point::new(x.try_into().unwrap(), y.try_into().unwrap()));
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

    pub fn render(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.frame_pos = [[0; 32];64];
        self.canvas.clear();
    }
}