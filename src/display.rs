/*
Original CHIP-8 display resolution is 64×32 pixels, and color is monochrome. 
Graphics are drawn to the screen solely by drawing sprites, 
which are 8 pixels wide and may be from 1 to 15 pixels in height. 
Sprite pixels are XOR'd with corresponding screen pixels. In other words, 
sprite pixels that are set flip the color of the corresponding screen pixel, while unset sprite pixels do nothing. 
The carry flag (VF) is set to 1 if any screen pixels are flipped from set to unset when a sprite is drawn and set to 0 otherwise. 
This is used for collision detection. 
 */

use crate::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT};
use crate::settings::DisplaySettings;
use std::sync::Arc;

// Display-specific constants that don't change
const RGBA_PIXEL_SIZE: usize = 4;          // Bytes per RGBA pixel

pub struct Display {
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT], // 64x32 pixels, 1 bit per pixel
    phosphor: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT], // Phosphor decay values (0-255)
    settings: Arc<DisplaySettings>,
}



impl Display {
    pub fn new() -> Self {
        Self::with_settings(Arc::new(DisplaySettings::default()))
    }
    
    pub fn with_settings(settings: Arc<DisplaySettings>) -> Self {
        Self { 
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            phosphor: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            settings,
        }
    }

    pub fn clear(&mut self) {
        self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

  


    pub fn render_to_buffer(&mut self, buffer: &mut [u8]) {
        // Convert 64x32 boolean display to RGBA pixel buffer with phosphor simulation
        // Each pixel is 4 bytes (RGBA) // How fast phosphor decays
        
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let pixel_index = (y * DISPLAY_WIDTH + x) * RGBA_PIXEL_SIZE;
                
                // Update phosphor decay
                if !self.display[y][x] && self.phosphor[y][x] > 0 {
                    self.phosphor[y][x] = self.phosphor[y][x].saturating_sub(self.settings.phosphor_decay_rate);
                }
                
                // Render based on phosphor value (not just on/off)
                let brightness = self.phosphor[y][x];
                
                // Classic green phosphor color with brightness
                buffer[pixel_index] = (brightness / self.settings.color.red_divisor).min(255);     // R (slight red)
                buffer[pixel_index + 1] = brightness / self.settings.color.green_divisor;          // G (full green)
                buffer[pixel_index + 2] = (brightness / self.settings.color.blue_divisor).min(255); // B (very slight blue)
                buffer[pixel_index + 3] = 255;                                                      // A (always opaque)
            }
        }
    }

    pub fn get_dimensions() -> (u32, u32) {
        (DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        self.display[y as usize][x as usize]
    }

    pub fn toggle_pixel(&mut self, x: u8, y: u8) {
        let x = x as usize;
        let y = y as usize;
        self.display[y][x] ^= true;
        
        // If pixel is now on, set phosphor to max
        if self.display[y][x] {
            self.phosphor[y][x] = self.settings.max_phosphor_value;
        }
        // If pixel turned off, phosphor will decay naturally
    }

}