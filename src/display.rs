use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct Display {
    pixels: pixels::Pixels,
    window: Window,
}

impl Display {
    pub fn new(size: (u32, u32), event_loop: &EventLoop<()>) -> Result<Self> {
        //winit stuff
        let window = {
            let size = LogicalSize::new(size.0 as f64, size.1 as f64);
            WindowBuilder::new()
                .with_title("CHIP-8")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(event_loop)?
        };
        //pixels stuff
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(size.0, size.1, surface_texture)?
        };

        Ok(Display { pixels, window })
    }

    pub fn redraw(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        self.pixels.resize_surface(size.width, size.height);
    }

    //takes a list of coordinates of bits to flip
    pub fn update(&mut self, updates: Vec<(usize, usize)>) {}
}
