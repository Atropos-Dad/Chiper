mod opcodes;
mod display;
mod timer;
mod cpu;
mod memory;
mod reg;
mod font;
mod input;
mod emulator;

use emulator::{Emulator, EmulatorConfig};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    println!("Starting CHIP-8 emulator...");
    
    let config = EmulatorConfig::default();
    let (window_width, window_height) = Emulator::window_dimensions(&config);
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("CHIP-8 Emulator")
        .with_inner_size(LogicalSize::new(window_width, window_height))
        .with_resizable(false)
        .build(&event_loop)?;

    let mut emulator = Emulator::new(&window, config)?;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if emulator.is_escape_pressed(&input) {
                    *control_flow = ControlFlow::Exit;
                } else {
                    emulator.handle_keyboard_input(&input);
                }
            }
            Event::RedrawRequested(_) => {
                emulator.update();
                
                if let Err(err) = emulator.render() {
                    eprintln!("Render failed: {err}");
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
                *control_flow = ControlFlow::Wait;
            }
            _ => {}
        }
    });
}