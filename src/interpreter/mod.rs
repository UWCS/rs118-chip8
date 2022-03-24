mod display;
mod input;

use crossbeam::atomic::AtomicCell;
use crossbeam::sync::WaitGroup;
use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::ControlFlow;
use winit_input_helper::WinitInputHelper;

pub type Display = [[u8; 64]; 32];
pub type Keys = [bool; 16];

pub trait Cpu: Sized + Send + 'static {
    fn step(&mut self, keys: &Keys) -> Option<Display>;
    fn speed(&self) -> Duration;

    fn run(mut self) -> ! {
        let (event_loop, window, mut pixels) =
            display::init().expect("Could not initialise display");

        let mut input = WinitInputHelper::new();

        //include a flag so we know if the current frame has been drawn, to avoid drawing it twicw
        let display = Arc::new(AtomicCell::new(([[0; 64]; 32], false)));
        let keys = Arc::new(AtomicCell::new([false; 16]));

        //used so CPU doesnt start until display is ready
        //cant start CPU after display because display has to be on the main thread and blocks it
        let wg = WaitGroup::new();

        thread::spawn({
            let wg = wg.clone();
            let display = display.clone();
            let keys = keys.clone();
            move || {
                wg.wait();
                loop {
                    let t0 = Instant::now();
                    //step the cpu, handle display updates
                    if let Some(update) = self.step(&keys.load()) {
                        display.store((update, false));
                    }

                    //sleep to make time steps uniform
                    if let Some(sleepy_time) = self.speed().checked_sub(Instant::now() - t0) {
                        thread::sleep(sleepy_time);
                    }
                }
            }
        });

        wg.wait();

        event_loop.run(move |event, _, control_flow| {
            let new_frame = display.load();
            if !new_frame.1 {
                display::update(&mut pixels, &new_frame.0);
            }

            if let Event::RedrawRequested(_) = event {
                if let Err(e) = pixels.render() {
                    eprintln!("Pixels rendering failure, caused by: {:?}", e.source());
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
                //handle keyboard input to emulator
                keys.swap(input::key_state(&input));

                // Resize the window
                if let Some(size) = input.window_resized() {
                    pixels.resize_surface(size.width, size.height);
                }
            }
            window.request_redraw();
        });
    }
}
