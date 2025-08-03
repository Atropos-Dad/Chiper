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

use crate::font;


pub struct Display {
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT], // 64x32 pixels, 1 bit per pixel
}

pub struct Sprite {
    width: u8,
    height: u8,
    data: [u8; 8],
}

impl Sprite {
    pub fn new(width: u8, height: u8, data: [u8; 8]) -> Self {
        Self { width, height, data }
    }
}


use crossterm::{
    cursor,
    style::Print,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, stdout};

impl Display {
    pub fn new() -> Self {
        Self { display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT] }
    }

    pub fn clear(&mut self) {
        // what if the fastest way to do this is to just set the whole array to false?
        self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

  

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &Sprite) -> bool{
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision. The interpreter reads n
        // bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen
        // at coordinates (Vx, Vy). Sprites are XOR’d onto the existing screen. If this causes any pixels to be erased,
        // VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of
        // the display, it wraps around to the opposite side of the screen.
        let width = sprite.width as usize;
        let height = sprite.height as usize;
        let mut collision = false;

        for row in 0..height {
            for col in 0..width {
                let sprite_pixel = (sprite.data[row] >> (7 - col)) & 1;
                let display_x = (x as usize + col) % DISPLAY_WIDTH;
                let display_y = (y as usize + row) % DISPLAY_HEIGHT;

                if sprite_pixel == 1 {
                    self.display[display_y][display_x] ^= true; // xor 
                    if self.display[display_y][display_x] == false { // if the pixel was already set, it will be unset now
                        collision = true;
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

    pub fn render(&self) -> io::Result<()> {
        let mut stdout = stdout();
        
        // Clear screen and move cursor to top
        stdout.execute(terminal::Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;
        
        // Draw each pixel as a character
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                if self.display[y][x] {
                    stdout.execute(Print("█"))?; // Full block
                } else {
                    stdout.execute(Print(" "))?; // Space
                }
            }
            stdout.execute(Print("\n"))?;
        }
        
        Ok(())
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        self.display[y as usize][x as usize]
    }

    pub fn toggle_pixel(&mut self, x: u8, y: u8) {
        self.display[y as usize][x as usize] ^= true;
    }

    pub fn create_character_sprite(character: char) -> Sprite {
        // Get the 5-byte font data for this character
        let font_data = font::char_to_sprite_data(character);
        
        // Create sprite data array (pad with zeros since font is 5 bytes, sprite can hold 8)
        let mut sprite_data = [0u8; 8];
        sprite_data[0..5].copy_from_slice(font_data);
        
        Sprite::new(FONT_CHAR_WIDTH, FONT_CHAR_HEIGHT, sprite_data) // CHIP-8 font characters are 4 pixels wide, 5 pixels tall
    }
}