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

/// The Interpreter's representation of the CHIP-8 display.
/// The display is 64x32 pixels, each pixel being either on or off, represented by a `0` or `1`, respectively.
pub type Display = [[u8; 64]; 32];

/// This type is how keyboard input is presented to the Interpreter.
/// Each of the 16 keys can either be down (`true`) or up (`false`).
pub type Keys = [bool; 16];

/// CHIP-8 interpreters can be built using this trait.
/// [`step`][Interpreter::step] should be implemented on a type representing a CHIP-8 Interpreter to run the interpreter one clock cycle at a time, such that calling it in a loop runs the interpreter.
pub trait Interpreter {
    /// Executes the next CHIP-8 Instruction, modifying the state of the virtual machine/CPU accordingly.
    /// This is the main driver function, running the interpreter one clock cycle at a time.
    /// # Return
    /// If the instruction modified the state of the display, then an updated [`Display`][Display] should be returned.
    /// # Panics
    /// Should panic if an unrecognised instruction is encountered
    fn step(&mut self, keys: &Keys) -> Option<Display>;

    /// Returns the duration of a single clock cycle, so the interpreter can keep the time steps uniform.
    /// See [`std::time`][std::time] for more information on [`Duration`][std::time::Duration].
    fn speed(&self) -> Duration;

    /// Indicates if the sound buzzer is currently active, such that the interpreter can handle sound accordingly.
    fn buzzer_active(&self) -> bool;
}

/// Starts the interpreter, blocking the current thread and running until killed.
/// Windowing, graphics, sound, and timing are all handled within this method.
pub fn run<I>(mut interpreter: I) -> !
where
    I: Interpreter + Send + 'static,
{
    let (event_loop, window, mut pixels) = display::init().expect("Could not initialise display");

    let mut input = WinitInputHelper::new();

    //include a flag so we know if the current frame has been drawn, to avoid drawing it twice
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
                if let Some(update) = interpreter.step(&keys.load()) {
                    display.store((update, false));
                }

                //sleep to make time steps uniform
                if let Some(sleepy_time) = interpreter.speed().checked_sub(Instant::now() - t0) {
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
