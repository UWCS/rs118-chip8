use std::io::Write;
use std::{thread, time};

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use anyhow::Result;

mod cpu;
mod vm;

fn main() -> Result<()> {
    //new event loop and input helper
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let size = (320, 320);
    //initialise our winit window
    let window = {
        let size = LogicalSize::new(size.0 as f64, size.1 as f64);
        WindowBuilder::new()
            .with_title("CHIP-8")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)?
    };

    //initialise our Pixels
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(size.0, size.1, surface_texture)?
    };

    //initialise our CPIU
    let mut cpu = cpu::CPU::init();
    cpu.load("IBM Logo.ch8")?;
    // const SPEED: u64 = 1428571; // 1/700Hz in nanoseconds
    const SPEED: u64 = 100_000_000;

    event_loop.run(move |event, _, control_flow| {
        let t0 = time::Instant::now();
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if let Some(interrupt) = cpu.tick() {
                match interrupt {
                    cpu::Interrupt::DisplayUpdate(buffer) => update(&mut pixels, buffer),
                }
                dbg!("updating display");
            }

            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
        }
        // request a redraw at the end of each loop
        window.request_redraw();
    });
}

pub fn update(pixels: &mut Pixels, buffer: [[u8; 64]; 32]) {
    pixels
        .get_frame()
        .write_all(&buffer.concat())
        .expect("Could not update Pixels buffer");
}
