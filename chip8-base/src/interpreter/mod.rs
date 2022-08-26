mod display;
mod input;
mod sound;

use crate::*;
use crossbeam::atomic::AtomicCell;
use crossbeam::sync::WaitGroup;
use std::error::Error;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::ControlFlow;
use winit_input_helper::WinitInputHelper;

/// Starts the interpreter, blocking the current thread and running until killed.
/// Windowing, graphics, sound, and timing are all handled within this method.
pub fn run<I>(mut interpreter: I) -> !
where
    I: Interpreter + Send + 'static,
{
    let (event_loop, window, mut pixels) = display::init().expect("Could not initialise display");

    let mut input = WinitInputHelper::new();

    //include a flag so we know if the current frame has been drawn, to avoid drawing it twice
    let frame_buffer = Arc::new(AtomicCell::new(([[Pixel::default(); 64]; 32], false)));
    let input_buffer = Arc::new(AtomicCell::new([false; 16]));

    //init the buzzer
    //as long as this is done first, we dont need to wait for it
    let buzzer = sound::Buzzer::init()
        .map_err(|err| eprintln!("Could not initalise sound: {err}, continuing with no buzzer"))
        .unwrap();

    //used so CPU doesnt start until display is ready
    //cant start CPU after display because display has to be on the main thread and blocks it
    let wg = WaitGroup::new();

    thread::spawn({
        //make copies of what we need
        let wg = wg.clone();
        let frame_buffer = frame_buffer.clone();
        let input_buffer = input_buffer.clone();

        //start thread
        move || {
            wg.wait(); //wait until event loop ready
            loop {
                let t0 = Instant::now();
                //step the cpu, read input buffer, write to framebuffer
                if let Some(update) = interpreter.step(&input_buffer.load()) {
                    frame_buffer.store((update, false));
                }

                //handle sound
                buzzer
                    .switch
                    .store(interpreter.buzzer_active(), Ordering::Relaxed);

                //sleep to make time steps uniform
                if let Some(sleepy_time) = interpreter.speed().checked_sub(Instant::now() - t0) {
                    thread::sleep(sleepy_time);
                }
            }
        }
    });

    //event loop starts here
    wg.wait(); //start other threads
    event_loop.run(move |event, _, control_flow| {
        let new_frame = frame_buffer.load();

        //only redraw if there was an update
        if !new_frame.1 {
            display::update(&mut pixels, &new_frame.0);
        }

        //if the OS requested a redraw of the window
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
            input_buffer.swap(input::key_state(&input));

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
        }
        window.request_redraw();
    });
}
