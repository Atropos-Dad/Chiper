/*
CHIP-8 Font Set
Each character is 4 pixels wide and 5 pixels tall.
Each character is represented as 5 bytes, with each byte representing a row.
Only the upper 4 bits of each byte are used.
Characters 0-F (hexadecimal digits)
*/

// Font constants
const FONT_SET_SIZE: usize = 80;           // Total font data size (16 chars × 5 bytes)
const FONT_CHAR_SIZE: usize = 5;           // Size of each font character in bytes
const HEX_DIGIT_OFFSET: u8 = 10;           // Offset for hex digits A-F

pub const FONT_SET: [u8; FONT_SET_SIZE] = [
    // Character '0'
    0xF0, // ****
    0x90, // *  *
    0x90, // *  *
    0x90, // *  *
    0xF0, // ****
    
    // Character '1'
    0x20, //   *
    0x60, //  **
    0x20, //   *
    0x20, //   *
    0x70, //  ***
    
    // Character '2'
    0xF0, // ****
    0x10, //    *
    0xF0, // ****
    0x80, // *
    0xF0, // ****
    
    // Character '3'
    0xF0, // ****
    0x10, //    *
    0xF0, // ****
    0x10, //    *
    0xF0, // ****
    
    // Character '4'
    0x90, // *  *
    0x90, // *  *
    0xF0, // ****
    0x10, //    *
    0x10, //    *
    
    // Character '5'
    0xF0, // ****
    0x80, // *
    0xF0, // ****
    0x10, //    *
    0xF0, // ****
    
    // Character '6'
    0xF0, // ****
    0x80, // *
    0xF0, // ****
    0x90, // *  *
    0xF0, // ****
    
    // Character '7'
    0xF0, // ****
    0x10, //    *
    0x20, //   *
    0x40, //  *
    0x40, //  *
    
    // Character '8'
    0xF0, // ****
    0x90, // *  *
    0xF0, // ****
    0x90, // *  *
    0xF0, // ****
    
    // Character '9'
    0xF0, // ****
    0x90, // *  *
    0xF0, // ****
    0x10, //    *
    0xF0, // ****
    
    // Character 'A'
    0xF0, // ****
    0x90, // *  *
    0xF0, // ****
    0x90, // *  *
    0x90, // *  *
    
    // Character 'B'
    0xE0, // ***
    0x90, // *  *
    0xE0, // ***
    0x90, // *  *
    0xE0, // ***
    
    // Character 'C'
    0xF0, // ****
    0x80, // *
    0x80, // *
    0x80, // *
    0xF0, // ****
    
    // Character 'D'
    0xE0, // ***
    0x90, // *  *
    0x90, // *  *
    0x90, // *  *
    0xE0, // ***
    
    // Character 'E'
    0xF0, // ****
    0x80, // *
    0xF0, // ****
    0x80, // *
    0xF0, // ****
    
    // Character 'F'
    0xF0, // ****
    0x80, // *
    0xF0, // ****
    0x80, // *
    0x80, // *
];

pub fn get_character_sprite(character: u8) -> &'static [u8] {
    let start_index = (character as usize) * FONT_CHAR_SIZE;
    &FONT_SET[start_index..start_index + FONT_CHAR_SIZE]
}

pub fn char_to_sprite_data(character: char) -> &'static [u8] {
    // Convert character to hex digit (0-F)
    let hex_value = match character {
        '0'..='9' => character as u8 - b'0',
        'A'..='F' => character as u8 - b'A' + HEX_DIGIT_OFFSET,
        'a'..='f' => character as u8 - b'a' + HEX_DIGIT_OFFSET,
        _ => 0, // Default to '0' for invalid characters
    };
    
    get_character_sprite(hex_value)
}
