mod opcodes;
mod display;
mod timer;
mod cpu;
mod memory;
mod reg;
mod font;
mod input;
mod emulator;
mod gif_recorder;

use emulator::{Emulator, EmulatorConfig};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{WindowEvent, ElementState};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use std::sync::Arc;
use winit::keyboard::{KeyCode, PhysicalKey};
use std::env;

struct Chip8App {
    window: Option<Arc<Window>>,
    emulator: Option<Emulator>,
    config: EmulatorConfig,
}

impl ApplicationHandler for Chip8App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let (window_width, window_height) = Emulator::window_dimensions(&self.config);
        
        let window = Arc::new(event_loop.create_window(
            Window::default_attributes()
                .with_title("CHIP-8 Emulator")
                .with_inner_size(LogicalSize::new(window_width, window_height))
                .with_resizable(false)
        ).unwrap());

        match Emulator::new(window.clone(), self.config.clone()) {
            Ok(emulator) => {
                self.emulator = Some(emulator);
                self.window = Some(window);
            }
            Err(e) => {
                eprintln!("Failed to create emulator: {}", e);
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if matches!(
                    (event.state, event.physical_key),
                    (ElementState::Pressed, PhysicalKey::Code(KeyCode::Escape))
                ) {
                    event_loop.exit();
                } else if let Some(emulator) = &mut self.emulator {
                    emulator.handle_keyboard_input(&event);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(emulator) = &mut self.emulator {
                    emulator.update();
                    
                    if let Err(err) = emulator.render() {
                        eprintln!("Render failed: {err}");
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

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
    println!("Controls: Press 'R' to start/stop GIF recording");
    
    let mut config = EmulatorConfig::default();
    config.rom_path = rom_path;

    let event_loop = EventLoop::new().unwrap();
    let mut app = Chip8App {
        window: None,
        emulator: None,
        config,
    };

    event_loop.run_app(&mut app).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}