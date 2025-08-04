// CHIP-8 memory layout constants
const MEMORY_SIZE: usize = 4096;           // Total memory size (4KB)
const MEMORY_SIZE_HEX: u16 = 0x1000;      // Memory size as hex  
const MEMORY_MAX_ADDRESS: u16 = 0xFFF;    // Maximum valid memory address

// 4096 (0x1000) memory locations, all of which are 8 bits (a byte) 
// which is where the term CHIP-8 originated. However, the CHIP-8 interpreter 
// itself occupies the first 512 bytes of the memory space on these machines. 
// For this reason, most programs written for the original system begin at memory location 512 (0x200) 
// and do not access any of the memory below the location 512 (0x200). The uppermost 256 bytes (0xF00-0xFFF) 
// /re reserved for display refresh, and the 96 bytes below that (0xEA0-0xEFF) were reserved for the call stack, 
// internal use, and other variables. 

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self { memory: [0; MEMORY_SIZE] } // initialize memory to 0!
    }

    pub fn read(&self, address: u16) -> u8 {
        if address < MEMORY_SIZE_HEX {
            self.memory[address as usize]
        } else {
            panic!("Memory read out of bounds: {}", address);
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        if address < MEMORY_MAX_ADDRESS {
            let hi = self.memory[address as usize] as u16;
            let lo = self.memory[(address + 1) as usize] as u16;
            (hi << 8) | lo
        } else {
            panic!("Memory read out of bounds: {}", address);
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if address < MEMORY_SIZE_HEX {
            self.memory[address as usize] = value;
        } else {
            panic!("Memory write out of bounds: {}", address);
        }
    }
    
}


pub struct rom_file {
    pub data: Vec<u8>,
}

impl rom_file {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn load_to_memory(&self, memory: &mut Memory, start_address: u16) {
        for (i, &byte) in self.data.iter().enumerate() {
            memory.write(start_address + i as u16, byte);
        }
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, std::io::Error> {
        let data = std::fs::read(file_path)?;
        Ok(Self { data })
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

pub struct Stack {
    stack: [Option<u16>; STACK_SIZE], // 48 bytes total, 13 bits to store a pointer so we need 16 bits total. We are going to emulate 12 for nesting tho.

}

impl Stack {
    pub fn new() -> Self {
        Self { stack: [None; STACK_SIZE] } // Use None for empty slots
    }

    pub fn pop(&mut self) -> Option<u16> {
        for i in (0..STACK_SIZE).rev() {
            if self.stack[i].is_some() {
                let value = self.stack[i].take(); // take() replaces with None
                return value;
            }
        }
        None
    }

    pub fn push(&mut self, value: u16) -> Result<(), &'static str> {
        for i in 0..STACK_SIZE {
            if self.stack[i].is_none() {
                self.stack[i] = Some(value);
                return Ok(());
            }
        }
        Err("Stack overflow")
    }
}