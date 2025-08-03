
use crate::memory::Memory;
use crate::memory::Stack;
use crate::reg::Registers;
use crate::timer::Timers;
use crate::display::{Display};
pub struct CPU {
    registers: Registers,
    memory: Memory,
    stack: Stack,
    timers: Timers,
    display: Display,
    program_counter: u16,

    
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
            stack: Stack::new(),
            timers: Timers::new(),
            display: Display::new(),
            program_counter: 0x200, // CHIP-8 programs start at 0x200
        }
    }

    pub fn tick(&mut self) {
        self.timers.tick();
        // Additional methods for CPU operations would go here
    }

    pub fn get_address_register(&self) -> u16 {
        self.registers.get_i()
    }

    pub fn set_address_register(&mut self, value: u16) {
        self.registers.set_i(value);
    }

    pub fn get_register(&self, index: u8) -> u8 {
        self.registers.get_v(index)
    }

    pub fn set_register(&mut self, index: u8, value: u8) {
        self.registers.set_v(index, value);
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        self.memory.read(address)
    }

    pub fn write_memory(&mut self, address: u16, value: u8) {
        self.memory.write(address, value);
    }

    pub fn pop_stack(&mut self) -> Option<u16> {
        self.stack.pop()
    }
    pub fn push_stack(&mut self, value: u16) {
        self.stack.push(value);
    }
}