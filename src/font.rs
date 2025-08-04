/*
CHIP-8 Font Set
Each character is 4 pixels wide and 5 pixels tall.
Each character is represented as 5 bytes, with each byte representing a row.
Only the upper 4 bits of each byte are used.
Characters 0-F (hexadecimal digits)
*/

pub const FONT_SET: [u8; 80] = [
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

