use sdl2::{render::Canvas, video::Window, pixels::Color, rect::Point, VideoSubsystem, Sdl};

pub struct Display {
    width: u8,
    height: u8,
    canvas: Canvas<Window>,
    frame_pos: [[u8;32];64],
    scale: usize,
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,

}

impl Display {
    pub fn new() -> Self {
        let scale: usize = 15;

        let sdl_context = sdl2::init().unwrap();
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
            sdl_context: sdl_context,
            video_subsystem: video,
        }
    }

    pub fn flip_pixel(&mut self, x: usize, y: usize)
    {
        self.frame_pos[x][y] ^= 1;
    
        if self.frame_pos[x][y] == 1 {
            self.canvas.set_draw_color(Color::RGB(255, 255, 255))
        }
        else {
            self.canvas.set_draw_color(Color::RGB(0, 0, 0))
        }

        self.canvas.draw_point(Point::new(x.try_into().unwrap(), y.try_into().unwrap()));
    }

    pub fn render(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }
}