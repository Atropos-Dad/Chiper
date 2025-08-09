/*

    NNN: address
    NN: 8-bit constant
    N: 4-bit constant
    X and Y: 4-bit register identifier
    PC : Program Counter
    I : 12bit register (For memory address) (Similar to void pointer);
    VN: One of the 16 available variables. N may be 0 to F (hexadecimal);

0NNN    Call        Calls machine code routine (RCA 1802 for COSMAC VIP) at address NNN. Not necessary for most ROMs.[24]
00E0    Display     disp_clear()     Clears the screen.[24]
00EE    Flow        return;          Returns from a subroutine.[24]
1NNN    Flow        goto NNN;        Jumps to address NNN.[24]
2NNN    Flow        *(0xNNN)()       Calls subroutine at NNN.[24]
3XNN    Cond        if (Vx == NN)    Skips the next instruction if VX equals NN (usually the next instruction is a jump to skip a code block).[24]
4XNN    Cond        if (Vx != NN)    Skips the next instruction if VX does not equal NN (usually the next instruction is a jump to skip a code block).[24]
5XY0    Cond        if (Vx == Vy)    Skips the next instruction if VX equals VY (usually the next instruction is a jump to skip a code block).[24]
6XNN    Const       Vx = NN          Sets VX to NN.[24]
7XNN    Const       Vx += NN         Adds NN to VX (carry flag is not changed).[24]
8XY0    Assig       Vx = Vy          Sets VX to the value of VY.[24]
8XY1    BitOp       Vx |= Vy         Sets VX to VX or VY. (bitwise OR operation).[24]
8XY2    BitOp       Vx &= Vy         Sets VX to VX and VY. (bitwise AND operation).[24]
8XY3[a] BitOp       Vx ^= Vy         Sets VX to VX xor VY.[24]
8XY4    Math        Vx += Vy         Adds VY to VX. VF is set to 1 when there's an overflow, and to 0 when there is not.[24]
8XY5    Math        Vx -= Vy         VY is subtracted from VX. VF is set to 0 when there's an underflow, and 1 when there is not. (i.e. VF set to 1 if VX >= VY and 0 if not).[24]
8XY6[a] BitOp       Vx >>= 1         Shifts VX to the right by 1, then stores the least significant bit of VX prior to the shift into VF.[b][24]
8XY7[a] Math        Vx = Vy - Vx     Sets VX to VY minus VX. VF is set to 0 when there's an underflow, and 1 when there is not. (i.e. VF set to 1 if VY >= VX).[24]
8XYE[a] BitOp       Vx <<= 1         Shifts VX to the left by 1, then sets VF to 1 if the most significant bit of VX prior to that shift was set, or to 0 if it was unset.[b][24]
9XY0    Cond        if (Vx != Vy)    Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block).[24]
ANNN    MEM         I = NNN          Sets I to the address NNN.[24]
BNNN    Flow        PC = V0 + NNN    Jumps to the address NNN plus V0.[24]
CXNN    Rand        Vx = rand() & NN Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.[24]
DXYN    Display     draw(Vx, Vy, N)  Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen.[24]
EX9E    KeyOp       if (key() == Vx) Skips the next instruction if the key stored in VX(only consider the lowest nibble) is pressed (usually the next instruction is a jump to skip a code block).[24]
EXA1    KeyOp       if (key() != Vx) Skips the next instruction if the key stored in VX(only consider the lowest nibble) is not pressed (usually the next instruction is a jump to skip a code block).[24]
FX07    Timer       Vx = get_delay() Sets VX to the value of the delay timer.[24]
FX0A    KeyOp       Vx = get_key()   A key press is awaited, and then stored in VX (blocking operation, all instruction halted until next key event, delay and sound timers should continue processing).[24]
FX15    Timer       delay_timer(Vx)  Sets the delay timer to VX.[24]
FX18    Sound       sound_timer(Vx)  Sets the sound timer to VX.[24]
FX1E    MEM         I += Vx          Adds VX to I. VF is not affected.[c][24]
FX29    MEM         I = sprite_addr[Vx]  Sets I to the location of the sprite for the character in VX(only consider the lowest nibble). Characters 0-F (in hexadecimal) are represented by a 4x5 font.[24]
FX33    BCD         

set_BCD(Vx)
*(I+0) = BCD(3);
*(I+1) = BCD(2);
*(I+2) = BCD(1);

	Stores the binary-coded decimal representation of VX, with the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.[24]
FX55 	MEM 	reg_dump(Vx, &I) 	Stores from V0 to VX (including VX) in memory, starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.[d][24]
FX65 	MEM 	reg_load(Vx, &I) 	Fills from V0 to VX (including VX) with values from memory, starting at address I. The offset from I is increased by 1 for each value read, but I itself is left unmodified.[d][24]
 */

use crate::constants::{INSTRUCTION_SIZE, FONT_START_ADDRESS};

// Opcode-specific constants
const VF_REGISTER_INDEX: u8 = 0xF;        // Index of VF register (flags)
const V0_REGISTER_INDEX: u8 = 0;          // Index of V0 register
const NIBBLE_MASK: u8 = 0xF;              // Mask for single nibble
const FONT_CHAR_SIZE: u16 = 5;            // Size of each font character in bytes
const BCD_HUNDREDS: u8 = 100;             // BCD hundreds divisor
const BCD_TENS: u8 = 10;                  // BCD tens divisor

pub enum Opcode {
    // opcode: u16, // 2 bytes, 16 bits total, big endian...
    CallRoutine, // 0NNN
    ClearDisplay {}, // 00E0
    Return {}, // 00EE
    Goto { address: u16 }, // 1NNN
    CallSubroutine { address: u16 }, // 2NNN
    SkipIfEqual { register: u8, value: u8 }, // 3XNN
    SkipIfNotEqual { register: u8, value: u8 }, // 4XNN
    SkipIfRegistersEqual { reg_x: u8, reg_y: u8 }, // 5XY0
    SetRegister { register: u8, value: u8 }, // 6XNN
    AddToRegister { register: u8, value: u8 }, // 7XNN
    AssignRegister { reg_x: u8, reg_y: u8 }, // 8XY0
    BitwiseOr { reg_x: u8, reg_y: u8 }, // 8XY1
    BitwiseAnd { reg_x: u8, reg_y: u8 }, // 8XY2
    BitwiseXor { reg_x: u8, reg_y: u8 }, // 8XY3
    AddRegisters { reg_x: u8, reg_y: u8 }, // 8XY4
    SubtractRegisters { reg_x: u8, reg_y: u8 }, // 8XY5
    ShiftRight { reg_x: u8 }, // 8XY6
    SubtractReverse { reg_x: u8, reg_y: u8 }, // 8XY7
    ShiftLeft { reg_x: u8 }, // 8XYE
    SkipIfRegNotEqual { reg_x: u8, reg_y: u8 }, // 9XY0
    SetAddress { address: u16 }, // ANNN
    JumpWithOffset { address: u16 }, // BNNN
    Random { register: u8, value: u8 }, // CXNN
    Draw { reg_x: u8, reg_y: u8, height: u8 }, // DXYN
    SkipIfKeyPressed { register: u8 }, // EX9E
    SkipIfKeyNotPressed { register: u8 }, // EXA1
    GetDelayTimer { register: u8 }, // FX07
    WaitForKey { register: u8 }, // FX0A
    SetDelayTimer { register: u8 }, // FX15
    SetSoundTimer { register: u8 }, // FX18
    AddToAddress { register: u8 }, // FX1E
    SetSpriteAddress { register: u8 }, // FX29
    StoreBCD { register: u8 }, // FX33
    StoreRegisters { reg_x: u8 }, // FX55
    LoadRegisters { reg_x: u8 }, // FX65
}

impl Opcode {
    pub fn from_raw(opcode: u16) -> Self {
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F),
        );
        
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let x = nibbles.1 as u8;
        let y = nibbles.2 as u8;
        let n = nibbles.3 as u8;
        
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => Opcode::ClearDisplay {},
            (0x0, 0x0, 0xE, 0xE) => Opcode::Return {},
            (0x0, _, _, _) => Opcode::CallRoutine,
            (0x1, _, _, _) => Opcode::Goto { address: nnn },
            (0x2, _, _, _) => Opcode::CallSubroutine { address: nnn },
            (0x3, _, _, _) => Opcode::SkipIfEqual { register: x, value: nn },
            (0x4, _, _, _) => Opcode::SkipIfNotEqual { register: x, value: nn },
            (0x5, _, _, 0x0) => Opcode::SkipIfRegistersEqual { reg_x: x, reg_y: y },
            (0x6, _, _, _) => Opcode::SetRegister { register: x, value: nn },
            (0x7, _, _, _) => Opcode::AddToRegister { register: x, value: nn },
            (0x8, _, _, 0x0) => Opcode::AssignRegister { reg_x: x, reg_y: y },
            (0x8, _, _, 0x1) => Opcode::BitwiseOr { reg_x: x, reg_y: y },
            (0x8, _, _, 0x2) => Opcode::BitwiseAnd { reg_x: x, reg_y: y },
            (0x8, _, _, 0x3) => Opcode::BitwiseXor { reg_x: x, reg_y: y },
            (0x8, _, _, 0x4) => Opcode::AddRegisters { reg_x: x, reg_y: y },
            (0x8, _, _, 0x5) => Opcode::SubtractRegisters { reg_x: x, reg_y: y },
            (0x8, _, _, 0x6) => Opcode::ShiftRight { reg_x: x },
            (0x8, _, _, 0x7) => Opcode::SubtractReverse { reg_x: x, reg_y: y },
            (0x8, _, _, 0xE) => Opcode::ShiftLeft { reg_x: x },
            (0x9, _, _, 0x0) => Opcode::SkipIfRegNotEqual { reg_x: x, reg_y: y },
            (0xA, _, _, _) => Opcode::SetAddress { address: nnn },
            (0xB, _, _, _) => Opcode::JumpWithOffset { address: nnn },
            (0xC, _, _, _) => Opcode::Random { register: x, value: nn },
            (0xD, _, _, _) => Opcode::Draw { reg_x: x, reg_y: y, height: n },
            (0xE, _, 0x9, 0xE) => Opcode::SkipIfKeyPressed { register: x },
            (0xE, _, 0xA, 0x1) => Opcode::SkipIfKeyNotPressed { register: x },
            (0xF, _, 0x0, 0x7) => Opcode::GetDelayTimer { register: x },
            (0xF, _, 0x0, 0xA) => Opcode::WaitForKey { register: x },
            (0xF, _, 0x1, 0x5) => Opcode::SetDelayTimer { register: x },
            (0xF, _, 0x1, 0x8) => Opcode::SetSoundTimer { register: x },
            (0xF, _, 0x1, 0xE) => Opcode::AddToAddress { register: x },
            (0xF, _, 0x2, 0x9) => Opcode::SetSpriteAddress { register: x },
            (0xF, _, 0x3, 0x3) => Opcode::StoreBCD { register: x },
            (0xF, _, 0x5, 0x5) => Opcode::StoreRegisters { reg_x: x },
            (0xF, _, 0x6, 0x5) => Opcode::LoadRegisters { reg_x: x },
            _ => panic!("Unknown opcode: {:#06X}", opcode),
        }
    }

    pub fn execute(&self, cpu: &mut crate::cpu::CPU) {
        use rand::Rng;
        
        match self {
            Opcode::CallRoutine => {
                unimplemented!("CallRoutine not implemented for modern CHIP-8")
            }
            Opcode::ClearDisplay {} => {
                cpu.clear_display();
            }
            Opcode::Return {} => {
                if let Some(addr) = cpu.pop_stack() {
                    cpu.set_program_counter(addr);
                }
            }
            Opcode::Goto { address } => {
                cpu.set_program_counter(*address);
            }
            Opcode::CallSubroutine { address } => {
                cpu.push_stack(cpu.get_program_counter());
                cpu.set_program_counter(*address);
            }
            Opcode::SkipIfEqual { register, value } => {
                if cpu.get_register(*register) == *value {
                    cpu.increment_program_counter();
                }
            }
            Opcode::SkipIfNotEqual { register, value } => {
                if cpu.get_register(*register) != *value {
                    cpu.increment_program_counter();
                }
            }
            Opcode::SkipIfRegistersEqual { reg_x, reg_y } => {
                if cpu.get_register(*reg_x) == cpu.get_register(*reg_y) {
                    cpu.increment_program_counter();
                }
            }
            Opcode::SetRegister { register, value } => {
                cpu.set_register(*register, *value);
            }
            Opcode::AddToRegister { register, value } => {
                let current = cpu.get_register(*register);
                cpu.set_register(*register, current.wrapping_add(*value));
            }
            Opcode::AssignRegister { reg_x, reg_y } => {
                let value = cpu.get_register(*reg_y);
                cpu.set_register(*reg_x, value);
            }
            Opcode::BitwiseOr { reg_x, reg_y } => {
                let x_val = cpu.get_register(*reg_x);
                let y_val = cpu.get_register(*reg_y);
                cpu.set_register(*reg_x, x_val | y_val);
            }
            Opcode::BitwiseAnd { reg_x, reg_y } => {
                let x_val = cpu.get_register(*reg_x);
                let y_val = cpu.get_register(*reg_y);
                cpu.set_register(*reg_x, x_val & y_val);
            }
            Opcode::BitwiseXor { reg_x, reg_y } => {
                let x_val = cpu.get_register(*reg_x);
                let y_val = cpu.get_register(*reg_y);
                cpu.set_register(*reg_x, x_val ^ y_val);
            }
            Opcode::AddRegisters { reg_x, reg_y } => {
                let x_val = cpu.get_register(*reg_x);
                let y_val = cpu.get_register(*reg_y);
                let (result, overflow) = x_val.overflowing_add(y_val);
                cpu.set_register(*reg_x, result);
                cpu.set_register(VF_REGISTER_INDEX, if overflow { 1 } else { 0 });
            }
            Opcode::SubtractRegisters { reg_x, reg_y } => {
                let x_val = cpu.get_register(*reg_x);
                let y_val = cpu.get_register(*reg_y);
                let (result, borrow) = x_val.overflowing_sub(y_val);
                cpu.set_register(*reg_x, result);
                cpu.set_register(VF_REGISTER_INDEX, if borrow { 0 } else { 1 });
            }
            Opcode::ShiftRight { reg_x } => {
                let value = cpu.get_register(*reg_x);
                cpu.set_register(VF_REGISTER_INDEX, value & 0x1);
                cpu.set_register(*reg_x, value >> 1);
            }
            Opcode::SubtractReverse { reg_x, reg_y } => {
                let x_val = cpu.get_register(*reg_x);
                let y_val = cpu.get_register(*reg_y);
                let (result, borrow) = y_val.overflowing_sub(x_val);
                cpu.set_register(*reg_x, result);
                cpu.set_register(VF_REGISTER_INDEX, if borrow { 0 } else { 1 });
            }
            Opcode::ShiftLeft { reg_x } => {
                let value = cpu.get_register(*reg_x);
                cpu.set_register(VF_REGISTER_INDEX, (value >> 7) & 0x1);
                cpu.set_register(*reg_x, value << 1);
            }
            Opcode::SkipIfRegNotEqual { reg_x, reg_y } => {
                if cpu.get_register(*reg_x) != cpu.get_register(*reg_y) {
                    cpu.increment_program_counter();
                }
            }
            Opcode::SetAddress { address } => {
                cpu.set_address_register(*address);
            }
            Opcode::JumpWithOffset { address } => {
                cpu.set_program_counter(*address + cpu.get_register(V0_REGISTER_INDEX) as u16);
            }
            Opcode::Random { register, value } => {
                let mut rng = rand::rng();
                let random: u8 = rng.random();
                cpu.set_register(*register, random & *value);
            }
            Opcode::Draw { reg_x, reg_y, height } => {
                let x = cpu.get_register(*reg_x);
                let y = cpu.get_register(*reg_y);
                let collision = cpu.draw_sprite(x, y, *height);
                cpu.set_register(VF_REGISTER_INDEX, if collision { 1 } else { 0 });
            }
            Opcode::SkipIfKeyPressed { register } => {
                let key = cpu.get_register(*register) & NIBBLE_MASK;
                if cpu.is_key_pressed(key) {
                    cpu.increment_program_counter();
                }
            }
            Opcode::SkipIfKeyNotPressed { register } => {
                let key = cpu.get_register(*register) & NIBBLE_MASK;
                if !cpu.is_key_pressed(key) {
                    cpu.increment_program_counter();
                }
            }
            Opcode::GetDelayTimer { register } => {
                cpu.set_register(*register, cpu.get_delay_timer());
            }
            Opcode::WaitForKey { register } => {
                if !cpu.wait_for_key(*register) {
                    // Still waiting for key, go back to execute this instruction again
                    cpu.set_program_counter(cpu.get_program_counter() - INSTRUCTION_SIZE);
                }
            }
            Opcode::SetDelayTimer { register } => {
                let value = cpu.get_register(*register);
                cpu.set_delay_timer(value);
            }
            Opcode::SetSoundTimer { register } => {
                let value = cpu.get_register(*register);
                cpu.set_sound_timer(value);
            }
            Opcode::AddToAddress { register } => {
                let current = cpu.get_address_register();
                let value = cpu.get_register(*register) as u16;
                cpu.set_address_register(current.wrapping_add(value));
            }
            Opcode::SetSpriteAddress { register } => {
                let sprite_idx = cpu.get_register(*register) & NIBBLE_MASK;
                cpu.set_address_register(FONT_START_ADDRESS + (sprite_idx as u16 * FONT_CHAR_SIZE));
            }
            Opcode::StoreBCD { register } => {
                let value = cpu.get_register(*register);
                let i = cpu.get_address_register();
                cpu.write_memory(i, value / BCD_HUNDREDS);
                cpu.write_memory(i + 1, (value % BCD_HUNDREDS) / BCD_TENS);
                cpu.write_memory(i + 2, value % 10);
            }
            Opcode::StoreRegisters { reg_x } => {
                let i = cpu.get_address_register();
                for idx in 0..=*reg_x {
                    let value = cpu.get_register(idx);
                    cpu.write_memory(i + idx as u16, value);
                }
            }
            Opcode::LoadRegisters { reg_x } => {
                let i = cpu.get_address_register();
                for idx in 0..=*reg_x {
                    let value = cpu.read_memory(i + idx as u16);
                    cpu.set_register(idx, value);
                }
            }
        }
    }
}

