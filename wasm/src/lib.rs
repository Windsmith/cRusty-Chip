use core::{chip8::Chip8, *};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

#[wasm_bindgen]
pub struct EmuWasm {
    chip8: Chip8,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl EmuWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<EmuWasm, JsValue> {
        let chip8 = Chip8::new();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();

        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Ok(EmuWasm { chip8, ctx })
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.chip8.tick();
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        self.chip8.tick_timers();
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.chip8.reset();
    }

    #[wasm_bindgen]
    pub fn key_down(&mut self, evt: KeyboardEvent, pressed: bool) {
        let key = evt.key();
        if let Some(k) = key2btn(&key) {
            self.chip8.key_down(k)
        }
    }

    pub fn key_up(&mut self, evt: KeyboardEvent, pressed: bool) {
        let key = evt.key();
        if let Some(k) = key2btn(&key) {
            self.chip8.key_up(k)
        }
    }

    #[wasm_bindgen]
    pub fn load_game(&mut self, data: Uint8Array) {
        self.chip8.load_program_to_memory(data.to_vec())
    }

    #[wasm_bindgen]
    pub fn draw_screen(&mut self, scale: usize) {
        let disp = self.chip8.get_display();
        for i in 0..SCREEN_WIDTH {
            for j in 0..SCREEN_HEIGHT {
                if disp[i][j] == 1 {
                    self.ctx.fill_rect(
                        (i * scale) as f64,
                        (j * scale) as f64,
                        scale as f64,
                        scale as f64,
                    );
                }
            }
        }
    }
}

fn key2btn(key: &str) -> Option<usize> {
    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ => None,
    }
}
