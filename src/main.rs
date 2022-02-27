use winit::event::VirtualKeyCode;
use winit::event_loop::{ControlFlow, EventLoop};

use winit_input_helper::WinitInputHelper;
mod cpu;
mod display;
fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut display = display::Display::new((320, 320), &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                display.resize(&size);
            }

            // Update internal state and request a redraw
            display.redraw();
        }
    });
}
