use winit::keyboard::KeyCode;
use std::collections::HashSet;

pub struct InputState {
    pressed_keys: HashSet<u8>,
    last_key_pressed: Option<u8>,
    waiting_for_key: bool,
    key_for_wait: Option<u8>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            last_key_pressed: None,
            waiting_for_key: false,
            key_for_wait: None,
        }
    }

    pub fn handle_key_press(&mut self, key: KeyCode) {
        if let Some(chip8_key) = self.map_key_to_chip8(key) {
            self.pressed_keys.insert(chip8_key);
            self.last_key_pressed = Some(chip8_key);
            
            // If we're waiting for a key, store it
            if self.waiting_for_key && self.key_for_wait.is_none() {
                self.key_for_wait = Some(chip8_key);
            }
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
        if !self.waiting_for_key {
            // Start waiting for a key
            self.waiting_for_key = true;
            self.key_for_wait = None;
            None
        } else if let Some(key) = self.key_for_wait {
            // A key was pressed while waiting, now wait for it to be released
            if !self.pressed_keys.contains(&key) {
                // Key has been released, we can return it
                self.waiting_for_key = false;
                self.key_for_wait = None;
                Some(key)
            } else {
                // Key is still pressed, keep waiting
                None
            }
        } else {
            // Still waiting for a key press
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