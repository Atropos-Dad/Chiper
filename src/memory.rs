// 4096 (0x1000) memory locations, all of which are 8 bits (a byte) 
// which is where the term CHIP-8 originated. However, the CHIP-8 interpreter 
// itself occupies the first 512 bytes of the memory space on these machines. 
// For this reason, most programs written for the original system begin at memory location 512 (0x200) 
// and do not access any of the memory below the location 512 (0x200). The uppermost 256 bytes (0xF00-0xFFF) 
// /re reserved for display refresh, and the 96 bytes below that (0xEA0-0xEFF) were reserved for the call stack, 
// internal use, and other variables. 

struct Memory {
    memory: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Self { memory: [0; 4096] } // initialize memory to 0!
    }
}


// im pretty sure an array like this is the best way to do it. There might be something about how 
// programs interface that we might need to consider, but that is for sure a future us problem.


/*
The stack is only used to store return addresses when subroutines are called. 
The original RCA 1802 version allocated 48 bytes for up to 12 levels of nesting;[21] 
modern implementations usually have more.[22][23] 
 */

// 4 bytes per nest, 12 levels of nesting, 48 bytes total
// a pointer in this space is 16 bits wide


const STACK_SIZE: usize = 12;

struct Stack {
    stack: [u16; STACK_SIZE], // 48 bytes total, 13 bits to store a pointer so we need 16 bits total. We are going to emulate 12 for nesting tho.

}

impl Stack {
    pub fn new() -> Self {
        Self { stack: [0; STACK_SIZE] } // initialize stack to 0!
    }
}