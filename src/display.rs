/*
Original CHIP-8 display resolution is 64×32 pixels, and color is monochrome. 
Graphics are drawn to the screen solely by drawing sprites, 
which are 8 pixels wide and may be from 1 to 15 pixels in height. 
Sprite pixels are XOR'd with corresponding screen pixels. In other words, 
sprite pixels that are set flip the color of the corresponding screen pixel, while unset sprite pixels do nothing. 
The carry flag (VF) is set to 1 if any screen pixels are flipped from set to unset when a sprite is drawn and set to 0 otherwise. 
This is used for collision detection. 
 */

// Display constants
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

// Font constants
const FONT_CHAR_WIDTH: u8 = 4;
const FONT_CHAR_HEIGHT: u8 = 5;

// Sprite constants
const SPRITE_DATA_SIZE: usize = 8;         // Maximum sprite data array size
const PIXEL_BIT_SHIFT: u8 = 7;             // Bit shift for pixel extraction

// Phosphor display constants
const MAX_PHOSPHOR_VALUE: u8 = 255;        // Maximum phosphor brightness
const PHOSPHOR_DECAY_RATE: u8 = 15;        // How fast phosphor decays per frame
const RGBA_PIXEL_SIZE: usize = 4;          // Bytes per RGBA pixel
const RED_CHANNEL_DIVISOR: u8 = 4;         // Red channel brightness divisor
const BLUE_CHANNEL_DIVISOR: u8 = 8;        // Blue channel brightness divisor
const ALPHA_CHANNEL_VALUE: u8 = 255;       // Alpha channel (fully opaque)

use crate::font;


pub struct Display {
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT], // 64x32 pixels, 1 bit per pixel
    phosphor: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT], // Phosphor decay values (0-255)
}

pub struct Sprite {
    width: u8,
    height: u8,
    data: [u8; SPRITE_DATA_SIZE],
}

impl Sprite {
    pub fn new(width: u8, height: u8, data: [u8; SPRITE_DATA_SIZE]) -> Self {
        Self { width, height, data }
    }
}


impl Display {
    pub fn new() -> Self {
        Self { 
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            phosphor: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

  

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &Sprite) -> bool{
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision. The interpreter reads n
        // bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen
        // at coordinates (Vx, Vy). Sprites are XOR'd onto the existing screen. If this causes any pixels to be erased,
        // VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of
        // the display, it wraps around to the opposite side of the screen.
        let width = sprite.width as usize;
        let height = sprite.height as usize;
        let mut collision = false;

        for row in 0..height {
            for col in 0..width {
                let sprite_pixel = (sprite.data[row] >> (PIXEL_BIT_SHIFT - col)) & 1;
                let display_x = (x as usize + col) % DISPLAY_WIDTH;
                let display_y = (y as usize + row) % DISPLAY_HEIGHT;

                if sprite_pixel == 1 {
                    let was_on = self.display[display_y][display_x];
                    self.display[display_y][display_x] ^= true; // xor 
                    
                    if was_on && !self.display[display_y][display_x] { 
                        // Pixel turned off - collision detected
                        collision = true;
                    } else if self.display[display_y][display_x] {
                        // Pixel turned on - set phosphor to max
                        self.phosphor[display_y][display_x] = MAX_PHOSPHOR_VALUE;
                    }
                }
            }
        }

        // return if collision (i.e. flag (VF) should be set)
        if collision {
            true
        } else {
            false
        }
    }

    pub fn render_to_buffer(&mut self, buffer: &mut [u8]) {
        // Convert 64x32 boolean display to RGBA pixel buffer with phosphor simulation
        // Each pixel is 4 bytes (RGBA) // How fast phosphor decays
        
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let pixel_index = (y * DISPLAY_WIDTH + x) * RGBA_PIXEL_SIZE;
                
                // Update phosphor decay
                if !self.display[y][x] && self.phosphor[y][x] > 0 {
                    self.phosphor[y][x] = self.phosphor[y][x].saturating_sub(PHOSPHOR_DECAY_RATE);
                }
                
                // Render based on phosphor value (not just on/off)
                let brightness = self.phosphor[y][x];
                
                // Classic green phosphor color with brightness
                buffer[pixel_index] = (brightness / RED_CHANNEL_DIVISOR).min(ALPHA_CHANNEL_VALUE);     // R (slight red)
                buffer[pixel_index + 1] = brightness;                                                   // G (full green)
                buffer[pixel_index + 2] = (brightness / BLUE_CHANNEL_DIVISOR).min(ALPHA_CHANNEL_VALUE); // B (very slight blue)
                buffer[pixel_index + 3] = ALPHA_CHANNEL_VALUE;                                          // A (always opaque)
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
            self.phosphor[y][x] = MAX_PHOSPHOR_VALUE;
        }
        // If pixel turned off, phosphor will decay naturally
    }

    pub fn create_character_sprite(character: char) -> Sprite {
        // Get the 5-byte font data for this character
        let font_data = font::char_to_sprite_data(character);
        
        // Create sprite data array (pad with zeros since font is 5 bytes, sprite can hold 8)
        let mut sprite_data = [0u8; SPRITE_DATA_SIZE];
        sprite_data[0..5].copy_from_slice(font_data);
        
        Sprite::new(FONT_CHAR_WIDTH, FONT_CHAR_HEIGHT, sprite_data) // CHIP-8 font characters are 4 pixels wide, 5 pixels tall
    }
}