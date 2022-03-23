use std::io::Write;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub fn init() -> Option<(EventLoop<()>, Window, Pixels)> {
    //new event loop and input helper
    let event_loop = EventLoop::new();
    let size = (640, 320);
    //initialise our winit window
    let window: Window = {
        let size = LogicalSize::new(size.0 as f64, size.1 as f64);
        WindowBuilder::new()
            .with_title("CHIP-8")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .ok()?
    };

    //initialise our Pixels
    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(64, 32, surface_texture).ok()?
    };
    Some((event_loop, window, pixels))
}

pub fn update(pixels: &mut Pixels, buffer: &[[u8; 64]; 32]) {
    let mut old_buf = pixels.get_frame();
    for px in buffer.concat() {
        old_buf
            .write_all(match px {
                0 => &[0_u8, 0_u8, 0_u8, 255_u8],
                1 => &[255_u8, 255_u8, 255_u8, 255_u8],
                _ => unreachable!(),
            })
            .expect("Could not update Pixels buffer");
    }
}
