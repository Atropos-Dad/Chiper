
use crate::memory::Memory;
use crate::memory::Stack;
use crate::reg::Registers;
use crate::timer::Timers;
use crate::display::{Display};
use crate::opcodes::Opcode;
use crate::input::InputState;

// CPU constants
const PROGRAM_START_ADDRESS: u16 = 0x200;  // CHIP-8 programs start at 0x200
const INSTRUCTION_SIZE: u16 = 2;            // Size of each instruction in bytes
const DISPLAY_HEIGHT: u8 = 32;              // Display height for wrapping
const DISPLAY_WIDTH: u8 = 64;               // Display width for wrapping
const SPRITE_WIDTH: u8 = 8;                 // Standard sprite width
const PIXEL_BIT_SHIFT: u8 = 7;              // Bit shift for pixel extraction

pub struct CPU {
    registers: Registers,
    memory: Memory,
    stack: Stack,
    timers: Timers,
    display: Display,
    input: InputState,
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
            input: InputState::new(),
            program_counter: PROGRAM_START_ADDRESS,
        }
    }

    pub fn tick(&mut self) {
        // fetch
        let raw_opcode = self.memory.read_u16(self.program_counter);

        // increment program counter
        self.program_counter += INSTRUCTION_SIZE;

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
        self.program_counter += INSTRUCTION_SIZE;
    }

    pub fn clear_display(&mut self) {
        self.display.clear();
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, height: u8) -> bool {
        let mut collision = false;
        let addr = self.registers.get_i();
        
        for row in 0..height {
            let sprite_byte = self.memory.read(addr + row as u16);
            let y_pos = (y + row) % DISPLAY_HEIGHT;
            
            for col in 0..SPRITE_WIDTH {
                let x_pos = (x + col) % DISPLAY_WIDTH;
                let pixel = (sprite_byte >> (PIXEL_BIT_SHIFT - col)) & 1;
                
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
        self.input.is_key_pressed(key)
    }

    pub fn wait_for_key(&mut self, register: u8) -> bool {
        if let Some(key) = self.input.wait_for_key() {
            self.registers.set_v(register, key);
            true // Key was pressed, continue execution
        } else {
            false // Still waiting for key, don't advance PC
        }
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

    pub fn render_to_buffer(&mut self, buffer: &mut [u8]) {
        self.display.render_to_buffer(buffer);
    }

    pub fn update_timers(&mut self) {
        self.timers.tick();
    }

    pub fn handle_key_press(&mut self, key: winit::event::VirtualKeyCode) {
        self.input.handle_key_press(key);
    }

    pub fn handle_key_release(&mut self, key: winit::event::VirtualKeyCode) {
        self.input.handle_key_release(key);
    }

    pub fn is_waiting_for_key(&self) -> bool {
        self.input.is_waiting_for_key()
    }
}