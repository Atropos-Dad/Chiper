mod opcodes;
mod display;
mod timer;
mod cpu;
mod memory;
mod reg;
mod font;

use std::time::{Duration, Instant};

fn main() {
    println!("Chiper!");
    let mut cpu = cpu::CPU::new();

    // Load the Maze ROM
    match memory::rom_file::load_from_file("Maze.ch8") {
        Ok(rom) => {
            println!("Loaded Maze.ch8 successfully!");
            // Load ROM into memory starting at 0x200
            for (i, byte) in rom.data.iter().enumerate() {
                cpu.write_memory(0x200 + i as u16, *byte);
            }
            cpu.set_program_counter(0x200); // Set PC to start of program
        }
        Err(e) => {
            eprintln!("Failed to load Maze.ch8: {}", e);
            return;
        }
    }

    println!("Running CHIP-8 emulator...");

    // Timing constants
    const TARGET_FPS: u64 = 60;
    const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / TARGET_FPS);
    const CYCLES_PER_FRAME: u32 = 10; // Run 10 instructions per frame (~600 Hz)

    let mut last_frame = Instant::now();

    // Main emulation loop
    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(last_frame);
        
        if elapsed >= FRAME_DURATION {
            // Run multiple CPU cycles per frame
            for _ in 0..CYCLES_PER_FRAME {
                cpu.tick();
            }
            
            // Render display at 60Hz
            if let Err(e) = cpu.render_display() {
                eprintln!("Display error: {}", e);
            }
            
            // Update timers at 60Hz
            cpu.update_timers();
            
            last_frame = now;
        }
    }
}

