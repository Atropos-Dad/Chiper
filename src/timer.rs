
pub struct Timers{
    delay_timer: u8,
    sound_timer: u8,
}

impl Timers {
    pub fn new() -> Self {
        Self {
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            // play sound??
            self.sound_timer -= 1;
        }
    }

    pub fn get_delay(&self) -> u8 {
        self.delay_timer
    }

    pub fn set_delay(&mut self, value: u8) {
        self.delay_timer = value;
    }

    pub fn set_sound(&mut self, value: u8) {
        self.sound_timer = value;
    }
}

