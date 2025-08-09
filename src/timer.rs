
use rodio::{OutputStreamBuilder, Sink, Source};
use std::time::Duration;
use crate::settings::AudioSettings;
use std::sync::Arc;

pub struct Timers{
    delay_timer: u8,
    sound_timer: u8,
    _stream: rodio::OutputStream,
    sink: Sink,
    settings: Arc<AudioSettings>,
}

impl Timers {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::with_settings(Arc::new(AudioSettings::default()))
    }
    
    pub fn with_settings(settings: Arc<AudioSettings>) -> Self {
        let _stream = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(_stream.mixer());
        Self {
            delay_timer: 0,
            sound_timer: 0,
            _stream,
            sink,
            settings,
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
                let source = rodio::source::SineWave::new(self.settings.beep_frequency_hz)
                    .take_duration(Duration::from_millis(100))
                    .amplify(self.settings.beep_volume);
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

