use winit::keyboard::KeyCode;
use std::collections::HashSet;

pub struct InputState {
    pressed_keys: HashSet<u8>,
    last_key_pressed: Option<u8>,
    waiting_for_key: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            last_key_pressed: None,
            waiting_for_key: false,
        }
    }

    pub fn handle_key_press(&mut self, key: KeyCode) {
        if let Some(chip8_key) = self.map_key_to_chip8(key) {
            self.pressed_keys.insert(chip8_key);
            self.last_key_pressed = Some(chip8_key);
        }
    }

    pub fn handle_key_release(&mut self, key: KeyCode) {
        if let Some(chip8_key) = self.map_key_to_chip8(key) {
            self.pressed_keys.remove(&chip8_key);
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn wait_for_key(&mut self) -> Option<u8> {
        if let Some(key) = self.last_key_pressed.take() {
            self.waiting_for_key = false;
            Some(key)
        } else {
            self.waiting_for_key = true;
            None
        }
    }

    pub fn is_waiting_for_key(&self) -> bool {
        self.waiting_for_key
    }

    fn map_key_to_chip8(&self, key: KeyCode) -> Option<u8> {
        match key {
            // CHIP-8 Keypad Layout:
            // 1 2 3 C
            // 4 5 6 D  
            // 7 8 9 E
            // A 0 B F
            //
            // QWERTY Mapping:
            // 1 2 3 4
            // Q W E R
            // A S D F
            // Z X C V
            KeyCode::Digit1 => Some(0x1),
            KeyCode::Digit2 => Some(0x2),
            KeyCode::Digit3 => Some(0x3),
            KeyCode::Digit4 => Some(0xC),
            
            KeyCode::KeyQ => Some(0x4),
            KeyCode::KeyW => Some(0x5),
            KeyCode::KeyE => Some(0x6),
            KeyCode::KeyR => Some(0xD),
            
            KeyCode::KeyA => Some(0x7),
            KeyCode::KeyS => Some(0x8),
            KeyCode::KeyD => Some(0x9),
            KeyCode::KeyF => Some(0xE),
            
            KeyCode::KeyZ => Some(0xA),
            KeyCode::KeyX => Some(0x0),
            KeyCode::KeyC => Some(0xB),
            KeyCode::KeyV => Some(0xF),
            
            _ => None,
        }
    }
}