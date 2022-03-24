mod display;
mod input;

use std::time::{Duration, Instant};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::ControlFlow;
use winit_input_helper::WinitInputHelper;

pub type Display = [[u8; 64]; 32];
pub type Keys = [bool; 16];

pub trait Chip8Cpu: Sized + 'static {
    fn step(&mut self, display: &mut Display, keys: &Keys);
    fn speed(&self) -> Duration;
}

pub struct Chip8VM<C> {
    cpu: C,
    display: Display,
    keys: Keys,
}

impl<C: Chip8Cpu> Chip8VM<C> {
    pub fn new(cpu: C) -> Self {
        Chip8VM {
            cpu,
            display: [[0; 64]; 32],
            keys: [false; 16],
        }
    }

    pub fn run(mut self) -> ! {
        let (event_loop, window, mut pixels) =
            display::init().expect("Could not initialise display");

        let mut input = WinitInputHelper::new();

        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                display::update(&mut pixels, &self.display);
            }
            // Handle input events
            if input.update(&event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                //handle keyboard input to emulator
                input::update_keys(&input, &mut self.keys);
                // Resize the window
                if let Some(size) = input.window_resized() {
                    pixels.resize_surface(size.width, size.height);
                }
            }
            //cpu step
            let t0 = Instant::now();
            self.cpu.step(&mut self.display, &self.keys);
            eprintln!("Time taken: {:?}", Instant::now() - t0);

            // request a redraw at the end of each loop
            window.request_redraw();
        });
    }
}
