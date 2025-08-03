
use crate::memory::Memory;
use crate::memory::Stack;
use crate::reg::Registers;
use crate::timer::Timers;
pub struct CPU {
    registers: Registers,
    memory: Memory,
    stack: Stack,
    timers: Timers,
    program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
            stack: Stack::new(),
            timers: Timers::new(),
            program_counter: 0x200, // CHIP-8 programs start at 0x200
        }
    }

    pub fn tick(&mut self) {
        self.timers.tick();
        // Additional methods for CPU operations would go here
    }
}