use crate::cpu::CPU;
use crate::gif_recorder::GifRecorder;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{KeyEvent, ElementState};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;
use std::sync::Arc;
use std::time::{Duration, Instant};

// Emulator constants
const PROGRAM_START_ADDRESS: u16 = 0x200;   // ROM loading start address
const DEFAULT_SCALE_FACTOR: u32 = 10;       // Default window scale factor
const DEFAULT_CYCLES_PER_FRAME: u32 = 10;   // Default CPU cycles per frame
const DEFAULT_TARGET_FPS: u32 = 60;          // Default target frames per second
const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;

#[derive(Clone)]
pub struct RecordingConfig {
    pub output_dir: String,
    pub filename_pattern: String, // e.g. "{rom_name}_{timestamp}"
    #[allow(dead_code)]
    pub auto_increment: bool, // For future use
}

impl Default for RecordingConfig {
    fn default() -> Self {
        Self {
            output_dir: ".".to_string(),
            filename_pattern: "chip8_{rom_name}_{timestamp}".to_string(),
            auto_increment: true,
        }
    }
}

#[derive(Clone)]
pub struct EmulatorConfig {
    pub scale_factor: u32,
    pub cycles_per_frame: u32,
    pub target_fps: u32,
    pub rom_path: String,
    pub recording: RecordingConfig,
}

impl Default for EmulatorConfig {
    fn default() -> Self {
        Self {
            scale_factor: DEFAULT_SCALE_FACTOR,
            cycles_per_frame: DEFAULT_CYCLES_PER_FRAME,
            target_fps: DEFAULT_TARGET_FPS,
            rom_path: String::new(),
            recording: RecordingConfig::default(),
        }
    }
}

pub struct Emulator {
    cpu: CPU,
    pixels: Pixels<'static>,
    config: EmulatorConfig,
    last_update: Instant,
    gif_recorder: GifRecorder,
    rom_name: String, // Store ROM name for filename generation
}

impl Emulator {
    pub fn new(window: Arc<Window>, config: EmulatorConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
        let (display_width, display_height) = crate::display::Display::get_dimensions();
        
        let pixels = pixels::PixelsBuilder::new(display_width, display_height, surface_texture)
            .enable_vsync(true)
            .build()?;

        let mut cpu = CPU::new();
        let rom_name = Self::extract_rom_name(&config.rom_path);
        
        // Load ROM only if a path is provided
        if !config.rom_path.is_empty() {
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
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("ROM file not found: {}", config.rom_path)
                    )));
                }
            }
        } else {
            eprintln!("No ROM file specified");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No ROM file specified"
            )));
        }

        Ok(Self {
            cpu,
            pixels,
            config,
            last_update: Instant::now(),
            gif_recorder: GifRecorder::new(),
            rom_name,
        })
    }

    fn extract_rom_name(rom_path: &str) -> String {
        if rom_path.is_empty() {
            return "no_rom".to_string();
        }
        
        std::path::Path::new(rom_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    // Recording methods
    pub fn toggle_recording(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if self.gif_recorder.is_recording() {
            self.gif_recorder.stop_recording()?;
            println!("GIF recording stopped");
            Ok(false)
        } else {
            let filename = GifRecorder::generate_filename(
                &self.rom_name,
                &self.config.recording.output_dir,
                &self.config.recording.filename_pattern
            );
            self.gif_recorder.start_recording(&filename)?;
            println!("Started GIF recording: {}", filename);
            Ok(true)
        }
    }

    pub fn handle_keyboard_input(&mut self, event: &KeyEvent) {
        if let PhysicalKey::Code(keycode) = event.physical_key {
            match event.state {
                ElementState::Pressed => {
                    match keycode {
                        KeyCode::KeyR => {
                            if let Err(e) = self.toggle_recording() {
                                eprintln!("Recording error: {}", e);
                            }
                        }
                        _ => self.cpu.handle_key_press(keycode),
                    }
                }
                ElementState::Released => self.cpu.handle_key_release(keycode),
            }
        }
    }


    pub fn update(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        let frame_duration = Duration::from_nanos(NANOSECONDS_PER_SECOND / self.config.target_fps as u64);
        
        if elapsed >= frame_duration {
            // Always execute at least one cycle per frame, even when waiting for key
            // This allows the wait_for_key instruction to check if a key was pressed
            for _ in 0..self.config.cycles_per_frame {
                self.cpu.tick();
            }
            
            self.cpu.update_timers();
            self.last_update = now;
        }
    }

    pub fn render(&mut self) -> Result<(), pixels::Error> {
        let frame = self.pixels.frame_mut();
        self.cpu.render_to_buffer(frame);
        
        // Record frame if GIF recording is active
        if self.gif_recorder.is_recording() {
            if let Err(e) = self.gif_recorder.add_frame(frame) {
                eprintln!("Failed to add frame to GIF: {}", e);
            }
        }
        
        self.pixels.render()
    }

    pub fn window_dimensions(config: &EmulatorConfig) -> (u32, u32) {
        let (display_width, display_height) = crate::display::Display::get_dimensions();
        (display_width * config.scale_factor, display_height * config.scale_factor)
    }
}