use core::chip8::Chip8;

use sdl2::{Sdl, event::Event, render::Canvas, video::Window, rect::Rect, pixels::Color, keyboard::Keycode};
use clap::Parser;

const WINDOW_WIDTH: u32 = 64;
const WINDOW_HEIGHT: u32 = 32;
const WINDOW_SCALE: u32 = 15;
const TICKS_BEFORE_DRAWING: u32 = 10;

#[derive(Parser)]
struct Cli {
    //Name of the game
    game: String,
}

fn main() {
    let args = Cli::parse();

    let mut chip8: Chip8 = Chip8::new();
    chip8.load_program(&args.game);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8 Emulator", WINDOW_WIDTH * WINDOW_SCALE, WINDOW_HEIGHT * WINDOW_SCALE)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                break 'gameloop;
                },
                Event::KeyDown{keycode: Some(key), ..} => {
                    if let Some(k) = key2btn(key) {
                        chip8.key_down(k);
                    }
                },
                Event::KeyUp{keycode: Some(key), ..} => {
                    if let Some(k) = key2btn(key) {
                        chip8.key_up(k);
                    }
                },
                _ => ()
            }
        }

        for _ in 0..TICKS_BEFORE_DRAWING { chip8.tick(); }
        chip8.tick_timers();
        draw_screen(&mut chip8, &mut canvas);
        
    }
}

pub fn draw_screen(chip8: &mut Chip8, canvas: &mut Canvas<Window>) { 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let mut x = 0;
    let mut y = 0;
    
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    
    for row in chip8.get_display() {
        for pixel in row {
            if pixel == 1 {canvas.fill_rect(Rect::new((x * WINDOW_SCALE).try_into().unwrap(), (y * WINDOW_SCALE).try_into().unwrap(), WINDOW_SCALE, WINDOW_SCALE));}
            y += 1;
        }
        x += 1;
        y = 0;
    }

    canvas.present();
}

fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

