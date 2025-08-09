mod constants;
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
mod settings;

use emulator::{Emulator, EmulatorConfig};
use settings::Settings;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{WindowEvent, ElementState};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};
use std::sync::Arc;
use winit::keyboard::{KeyCode, PhysicalKey, ModifiersState};
use std::env;

struct Chip8App {
    window: Option<Arc<Window>>,
    emulator: Option<Emulator>,
    config: EmulatorConfig,
    modifiers: ModifiersState,
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
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = new_modifiers.state();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if matches!(
                    (event.state, event.physical_key),
                    (ElementState::Pressed, PhysicalKey::Code(KeyCode::Escape))
                ) {
                    event_loop.exit();
                } else if let Some(emulator) = &mut self.emulator {
                    emulator.handle_keyboard_input(&event, &self.modifiers);
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
        println!("Usage: {} <rom_file> [config_file]", args[0]);
        println!("  rom_file:    Path to CHIP-8 ROM file (.ch8)");
        println!("  config_file: Optional path to config file (.toml)");
        println!();
        println!("Example: {} PONG.ch8", args[0]);
        println!("Example: {} PONG.ch8 chip8_config.toml", args[0]);
        return Ok(());
    }
    
    let rom_path = args[1].clone();
    
    // Load settings from config file if provided
    let settings = if args.len() >= 3 {
        let config_file = &args[2];
        println!("Loading config from: {}", config_file);
        match Settings::load_from_file(config_file) {
            Ok(s) => {
                println!("Config loaded successfully.");
                s
            },
            Err(e) => {
                eprintln!("Failed to load config file: {}. Using defaults.", e);
                Settings::default()
            }
        }
    } else {
        // Check if default config file exists
        if std::path::Path::new("chip8_config.toml").exists() {
            println!("Found chip8_config.toml, loading...");
            match Settings::load_from_file("chip8_config.toml") {
                Ok(s) => {
                    println!("Config loaded successfully.");
                    s
                },
                Err(e) => {
                    eprintln!("Failed to load default config: {}. Using defaults.", e);
                    Settings::default()
                }
            }
        } else {
            println!("No config file specified. Using default settings.");
            Settings::default()
        }
    };
    
    println!("Starting CHIP-8 emulator with ROM: {}", rom_path);
    println!("Controls: Press 'Ctrl+R' to start/stop GIF recording");
    
    let config = EmulatorConfig {
        rom_path,
        settings,
    };

    let event_loop = EventLoop::new().unwrap();
    let mut app = Chip8App {
        window: None,
        emulator: None,
        config,
        modifiers: ModifiersState::empty(),
    };

    event_loop.run_app(&mut app).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}