mod display;
mod input;
mod sound;

use crate::{Interpreter, Pixel};
use anyhow::Context;
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
    //init display subsystem
    log::info!("Initalising display components...");
    let (event_loop, window, mut pixels) = display::init()
        .context("Could not initialise display subsystem.")
        .unwrap(); //failure to init display is fatal, so panic.

    //init input subsystem
    log::info!("Initalising input components...");
    let mut input = WinitInputHelper::new();

    //include a flag so we know if the current frame has been drawn, to avoid drawing it twice
    let frame_buffer = Arc::new(AtomicCell::new(([[Pixel::default(); 64]; 32], false)));
    let input_buffer = Arc::new(AtomicCell::new([false; 16]));

    //used so CPU doesnt start until display is ready
    //cant start CPU after display because display has to be on the main thread and blocks it
    let wg = WaitGroup::new();

    let handle = thread::Builder::new().name("VM Executor".to_string()).spawn({
        //make copies of what we need
        let wg = wg.clone();
        let frame_buffer = frame_buffer.clone();
        let input_buffer = input_buffer.clone();

        //start thread
        move || {
            //init the audio on the thread because cpal::stream:  !send
            log::info!("Initalising audio components...");
            let buzzer = sound::Buzzer::init()
                .map_err(|e| {
                    log::error!("Failure in initalising audio: {e:?}. Continuing with no sound.")
                })
                .ok();

            log::info!("Starting CPU...");
            wg.wait(); //wait until event loop ready
            loop {
                let t0 = Instant::now();
                //step the cpu, read input buffer, write to framebuffer
                if let Some(update) = interpreter.step(&input_buffer.load()) {
                    frame_buffer.store((update, false));
                }

                //handle sound
                if let Some(buzzer) = &buzzer {
                    buzzer.switch.store(interpreter.buzzer_active(), Ordering::Relaxed);
                }

                //sleep to make time steps uniform
                if let Some(sleepy_time) = interpreter.speed().checked_sub(Instant::now() - t0) {
                    thread::sleep(sleepy_time);
                    log::debug!(
                        "Took {:?} to execute instruction",
                        interpreter.speed() - sleepy_time
                    )
                } else {
                    log::warn!("CPU clock is running slow, your interpreter is taking too long to execute instructions.")
                }
            }
        }
    }).context("Could not start VM execution thread").unwrap();

    //event loop starts here
    wg.wait(); //start other thread
    log::info!("Starting input & display event loop...");

    event_loop.run(move |event, _, control_flow| {
        //if cpu thread has exited (due to panic), exit
        if handle.is_finished() {
            log::error!("VM thread has exited, shutting down...");
            *control_flow = ControlFlow::Exit;
            return;
        }

        let new_frame = frame_buffer.load();

        //only redraw if there was an update
        if !new_frame.1 {
            display::update(&mut pixels, &new_frame.0)
                .context("Failed to update display")
                .unwrap(); //panic if failed to update display for whatever reason
        }

        //if the OS requested a redraw of the window
        if let Event::RedrawRequested(_) = event {
            if let Err(e) = pixels.render() {
                panic!("Pixels rendering failure, caused by: {:?}", e.source());
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
