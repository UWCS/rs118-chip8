use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::ControlFlow;
use winit_input_helper::WinitInputHelper;
mod display;

#[derive(Clone)]
pub struct Display(Arc<RwLock<[[u8; 64]; 32]>>);

impl Display {
    pub fn get_buffer(&self) -> [[u8; 64]; 32] {
        *self.0.read().unwrap()
    }

    pub fn write_buffer(&self, new_buf: &[[u8; 64]; 32]) {
        let mut buf: [[u8; 64]; 32] = *self.0.write().unwrap();
        &mut buf.copy_from_slice(new_buf);
    }
}

#[derive(Clone)]
pub struct Keys(Arc<RwLock<[bool; 16]>>);

pub trait Chip8Cpu: Sized + Send + 'static {
    fn step(&mut self, display: &mut Display, keys: &Keys);
}

pub struct Chip8VM<C: Chip8Cpu> {
    cpu: C,
    display: Display,
    keys: Keys,
}

impl<C: Chip8Cpu> Chip8VM<C> {
    pub fn new(cpu: C) -> Self {
        Chip8VM {
            cpu,
            display: Display(Arc::new(RwLock::new([[0; 64]; 32]))),
            keys: Keys(Arc::new(RwLock::new([false; 16]))),
        }
    }

    pub fn run(mut self) -> ! {
        let (event_loop, window, mut pixels) =
            display::init().expect("Could not initialise display");

        let mut input = WinitInputHelper::new();

        let (tx, rx) = mpsc::channel();
        let mut tx = Some(tx);

        //start our CPU thread
        {
            let mut display = self.display.clone();
            let keys = self.keys.clone();
            thread::spawn(move || {
                let _ = rx.recv().unwrap(); //will block until read
                loop {
                    self.cpu.step(&mut display, &keys);
                }
            });
        }

        let display = self.display.clone();
        let keys = self.keys.clone();
        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                display::update(&mut pixels, &display.get_buffer());
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

            //at the end of the first iteration, start our CPU
            if let Some(tx) = tx.take() {
                tx.send(()).unwrap()
            }
        });
    }
}
