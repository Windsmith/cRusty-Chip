pub mod renderer {
    use pixels::{Pixels, SurfaceTexture};
    use winit::{
        dpi::LogicalSize,
        event::{Event, VirtualKeyCode},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    };

    const ROWS: u32 = 32;
    const COLS: u32 = 64;
    pub struct Renderer {
        rows: u32,
        columns: u32,
        scale: u32,
        pub event_loop: EventLoop<()>,
        pub window: Window,
        pub pixels: Pixels,
    }

    impl Renderer {
        pub fn new(scale: u32) -> Renderer {
            let width = ROWS * scale;
            let height = COLS * scale;

            let event_loop = EventLoop::new();

            let window = {
                let size = LogicalSize::new(COLS as f64, ROWS as f64);
                let scaled_size =
                    LogicalSize::new(COLS as f64 * scale as f64, ROWS as f64 * scale as f64);
                WindowBuilder::new()
                    .with_title("cRusty Chip")
                    .with_inner_size(scaled_size)
                    .with_min_inner_size(size)
                    .build(&event_loop)
                    .unwrap()
            };

            let surface_texture = SurfaceTexture::new(width, height, &window);

            Renderer {
                rows: ROWS,
                columns: COLS,
                scale,
                event_loop,
                pixels: Pixels::new(width, height, surface_texture).unwrap(),
                window: window,
            }
        }
    }
}
