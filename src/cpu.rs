
use crate::memory::Memory;
use crate::memory::Stack;
use crate::reg::Registers;
use crate::timer::Timers;
use crate::display::{Display};
use crate::opcodes::Opcode;
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


        // fetch
        let raw_opcode = self.memory.read_u16(self.program_counter);

        // increment program counter
        self.program_counter += 2;


        // decode
        let opcode: Opcode = Opcode::from_raw(raw_opcode);

        // execute
        opcode.execute(self);
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

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn set_program_counter(&mut self, value: u16) {
        self.program_counter = value;
    }

    pub fn increment_program_counter(&mut self) {
        self.program_counter += 2;
    }

    pub fn clear_display(&mut self) {
        self.display.clear();
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, height: u8) -> bool {
        let mut collision = false;
        let addr = self.registers.get_i();
        
        for row in 0..height {
            let sprite_byte = self.memory.read(addr + row as u16);
            let y_pos = (y + row) % 32;
            
            for col in 0..8 {
                let x_pos = (x + col) % 64;
                let pixel = (sprite_byte >> (7 - col)) & 1;
                
                if pixel == 1 {
                    if self.display.get_pixel(x_pos, y_pos) {
                        collision = true;
                    }
                    self.display.toggle_pixel(x_pos, y_pos);
                }
            }
        }
        
        collision
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        // TODO: Implement keyboard input
        false
    }

    pub fn wait_for_key(&mut self, register: u8) {
        // TODO: Implement keyboard wait
        // For now, just set a default value
        self.registers.set_v(register, 0);
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.timers.get_delay()
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.timers.set_delay(value);
    }

    pub fn set_sound_timer(&mut self, value: u8) {
        self.timers.set_sound(value);
    }

    pub fn render_display(&self) -> std::io::Result<()> {
        self.display.render()
    }

    pub fn update_timers(&mut self) {
        self.timers.tick();
    }
}