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
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("CHIP-8 Emulator");
        println!("Usage: {} <rom_file>", args[0]);
        println!("  rom_file: Path to CHIP-8 ROM file (.ch8)");
        println!();
        println!("Example: {} PONG.ch8", args[0]);
        return Ok(());
    }
    
    let rom_path = args[1].clone();
    println!("Starting CHIP-8 emulator with ROM: {}", rom_path);
    
    let mut config = EmulatorConfig::default();
    config.rom_path = rom_path;
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