
use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

pub struct Timers{
    delay_timer: u8,
    sound_timer: u8,
    _stream: OutputStream,
    sink: Sink,
}

impl Timers {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            delay_timer: 0,
            sound_timer: 0,
            _stream,
            sink,
        }
    }

    pub fn tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            // Play beep sound while timer is active
            if self.sink.empty() {
                let source = rodio::source::SineWave::new(440.0)
                    .take_duration(Duration::from_millis(100))
                    .amplify(0.05);
                self.sink.append(source);
                self.sink.play();
            }
        } else {
            // Stop sound when timer reaches 0
            self.sink.stop();
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

