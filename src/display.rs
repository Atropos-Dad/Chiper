/*
Original CHIP-8 display resolution is 64×32 pixels, and color is monochrome. 
Graphics are drawn to the screen solely by drawing sprites, 
which are 8 pixels wide and may be from 1 to 15 pixels in height. 
Sprite pixels are XOR'd with corresponding screen pixels. In other words, 
sprite pixels that are set flip the color of the corresponding screen pixel, while unset sprite pixels do nothing. 
The carry flag (VF) is set to 1 if any screen pixels are flipped from set to unset when a sprite is drawn and set to 0 otherwise. 
This is used for collision detection. 
 */


struct Display {
    display: [[bool; 64]; 32], // 64x32 pixels, 1 bit per pixel
}

struct Sprite {
    width: u8,
    height: u8,
    data: [u8; 8],
}

impl Sprite {
    pub fn new(width: u8, height: u8, data: [u8; 8]) -> Self {
        Self { width, height, data }
    }
}


impl Display {
    pub fn new() -> Self {
        Self { display: [[false; 64]; 32] }
    }

    pub fn clear(&mut self) {
        // what if the fastest way to do this is to just set the whole array to false?
        self.display = [[false; 64]; 32];
    }

    // pub fn clear(&mut self) {
    //     // loop through the display and set each pixel to false
    //     for row in self.display.iter_mut() {
    //         for pixel in row.iter_mut() {
    //             *pixel = false;
    //         }
    //     }
    // }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &Sprite) {
        // we need to draw the sprite at the given x and y coordinates
        // we need to check if the sprite is out of bounds
        // we need to check if the sprite is colliding with the screen
    }
}