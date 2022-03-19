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
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(size.0, size.1, surface_texture).ok()?
    };
    Some((event_loop, window, pixels))
}
