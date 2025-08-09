//! Shared architectural constants for the CHIP-8 emulator.
//! 
//! These constants define the fundamental aspects of the CHIP-8 architecture
//! that are shared across multiple modules and should not be changed.

/// Display dimensions - CHIP-8 has a fixed 64x32 monochrome display
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

/// Memory layout constants
pub const PROGRAM_START_ADDRESS: u16 = 0x200;  // Programs are loaded starting at 0x200
pub const FONT_START_ADDRESS: u16 = 0x50;      // Font data is stored starting at 0x50
pub const INSTRUCTION_SIZE: u16 = 2;           // Each instruction is 2 bytes