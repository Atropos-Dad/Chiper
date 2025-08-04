use crate::cpu::CPU;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{KeyboardInput, ElementState, VirtualKeyCode};
use winit::window::Window;
use std::time::{Duration, Instant};

// Emulator constants
const PROGRAM_START_ADDRESS: u16 = 0x200;   // ROM loading start address
const DEFAULT_SCALE_FACTOR: u32 = 10;       // Default window scale factor
const DEFAULT_CYCLES_PER_FRAME: u32 = 10;   // Default CPU cycles per frame
const DEFAULT_TARGET_FPS: u32 = 60;          // Default target frames per second
const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;

pub struct EmulatorConfig {
    pub scale_factor: u32,
    pub cycles_per_frame: u32,
    pub target_fps: u32,
    pub rom_path: String,
}

impl Default for EmulatorConfig {
    fn default() -> Self {
        Self {
            scale_factor: DEFAULT_SCALE_FACTOR,
            cycles_per_frame: DEFAULT_CYCLES_PER_FRAME,
            target_fps: DEFAULT_TARGET_FPS,
            rom_path: "PONG.ch8".to_string(),
        }
    }
}

pub struct Emulator {
    cpu: CPU,
    pixels: Pixels,
    config: EmulatorConfig,
    last_update: Instant,
}

impl Emulator {
    pub fn new(window: &Window, config: EmulatorConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        let (display_width, display_height) = crate::display::Display::get_dimensions();
        
        let pixels = pixels::PixelsBuilder::new(display_width, display_height, surface_texture)
            .enable_vsync(true)
            .build()?;

        let mut cpu = CPU::new();
        
        // Load ROM
        match crate::memory::RomFile::load_from_file(&config.rom_path) {
            Ok(rom) => {
                println!("Loaded {} successfully!", config.rom_path);
                for (i, byte) in rom.data.iter().enumerate() {
                    cpu.write_memory(PROGRAM_START_ADDRESS + i as u16, *byte);
                }
                cpu.set_program_counter(PROGRAM_START_ADDRESS);
            }
            Err(e) => {
                eprintln!("Failed to load {}: {}", config.rom_path, e);
            }
        }

        Ok(Self {
            cpu,
            pixels,
            config,
            last_update: Instant::now(),
        })
    }

    pub fn handle_keyboard_input(&mut self, input: &KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            match input.state {
                ElementState::Pressed => self.cpu.handle_key_press(keycode),
                ElementState::Released => self.cpu.handle_key_release(keycode),
            }
        }
    }

    pub fn is_escape_pressed(&self, input: &KeyboardInput) -> bool {
        matches!(
            (input.state, input.virtual_keycode),
            (ElementState::Pressed, Some(VirtualKeyCode::Escape))
        )
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        let frame_duration = Duration::from_nanos(NANOSECONDS_PER_SECOND / self.config.target_fps as u64);
        
        if elapsed >= frame_duration {
            if !self.cpu.is_waiting_for_key() {
                for _ in 0..self.config.cycles_per_frame {
                    self.cpu.tick();
                }
            }
            
            self.cpu.update_timers();
            self.last_update = now;
        }
    }

    pub fn render(&mut self) -> Result<(), pixels::Error> {
        let frame = self.pixels.frame_mut();
        self.cpu.render_to_buffer(frame);
        self.pixels.render()
    }

    pub fn window_dimensions(config: &EmulatorConfig) -> (u32, u32) {
        let (display_width, display_height) = crate::display::Display::get_dimensions();
        (display_width * config.scale_factor, display_height * config.scale_factor)
    }
}