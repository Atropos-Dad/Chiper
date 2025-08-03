/* CHIP-8 has 16 8-bit data registers named V0 to VF. The VF register doubles as a flag for some instructions; 
thus, it should be avoided. In an addition operation, VF is the carry flag, while in subtraction, it is the "no borrow" flag.
 In the draw instruction VF is set upon pixel collision. */

//  The address register, which is named I, is 12 bits wide and is used with several opcodes that involve memory operations. 

const DATA_REGISTERS_SIZE: usize = 16; // how many data registers we have

struct Registers {
    data_registers: [u8; DATA_REGISTERS_SIZE],
    address_register: u16, // 12 bits wide, can we enforce this?
}

impl Registers {
    pub fn new() -> Self {
        Self { data_registers: [0; DATA_REGISTERS_SIZE], address_register: 0 } // initialize registers to 0!
    }

    pub fn get_i(&self) -> u16 {
        self.address_register
    }

    pub fn set_i(&mut self, value: u16) {
        if value > 0xFFF {
            panic!("Address register value out of bounds: {}", value);
        }
         // Ensure the value is within the 12-bit range
        self.address_register = value;
        
    }

    pub fn get_v(&self, index: u8) -> u8 {
        if (index as usize) < DATA_REGISTERS_SIZE {
            self.data_registers[index as usize]
        } else {
            panic!("Index out of bounds for data registers: {}", index);
        }
    }

    pub fn set_v(&mut self, index: u8, value: u8) {
        if (index as usize) < DATA_REGISTERS_SIZE {
            self.data_registers[index as usize] = value;
        } else {
            panic!("Index out of bounds for data registers: {}", index);
        }
}
}