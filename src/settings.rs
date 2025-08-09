//! User-configurable settings for the CHIP-8 emulator.
//! 
//! These settings control various aspects of the emulator that users might want
//! to customize for their experience, including display effects, audio, and timing.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub display: DisplaySettings,
    pub audio: AudioSettings,
    pub cpu: CpuSettings,
    pub recording: RecordingSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DisplaySettings {
    /// Phosphor decay rate - how quickly the phosphor effect fades (0-255)
    pub phosphor_decay_rate: u8,
    
    /// Maximum phosphor brightness value
    pub max_phosphor_value: u8,
    
    /// Default window scale factor - how much to scale up the 64x32 display
    pub default_scale_factor: u32,
    
    /// Color settings for the phosphor display
    pub color: ColorSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ColorSettings {
    /// Red channel brightness divisor (1 = full brightness, higher = dimmer)
    pub red_divisor: u8,
    
    /// Green channel brightness divisor (1 = full brightness, higher = dimmer)
    pub green_divisor: u8,
    
    /// Blue channel brightness divisor (1 = full brightness, higher = dimmer)
    pub blue_divisor: u8,
    
    /// Background color (RGBA)
    pub background: [u8; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioSettings {
    /// Frequency of the beep sound in Hz (440 Hz = A4 note)
    pub beep_frequency_hz: f32,
    
    /// Audio volume/amplitude (0.0 to 1.0)
    pub beep_volume: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CpuSettings {
    /// Target frames per second for emulation
    pub target_fps: u32,
    
    /// Number of CPU cycles to execute per frame
    pub cycles_per_frame: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RecordingSettings {
    /// Scale factor for GIF recordings
    pub gif_scale_factor: u16,
    
    /// Delay between GIF frames in centiseconds
    pub gif_frame_delay: u16,
    
    /// Record every Nth frame
    pub gif_frame_skip: u32,
    
    /// Default output directory
    pub output_dir: String,
}

// Default implementations
impl Default for Settings {
    fn default() -> Self {
        Self {
            display: DisplaySettings::default(),
            audio: AudioSettings::default(),
            cpu: CpuSettings::default(),
            recording: RecordingSettings::default(),
        }
    }
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            phosphor_decay_rate: 15,
            max_phosphor_value: 255,
            default_scale_factor: 10,
            color: ColorSettings::default(),
        }
    }
}

impl Default for ColorSettings {
    fn default() -> Self {
        Self {
            red_divisor: 4,
            green_divisor: 1,
            blue_divisor: 8,
            background: [0, 0, 0, 255], // Black
        }
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            beep_frequency_hz: 440.0,
            beep_volume: 0.05,
        }
    }
}

impl Default for CpuSettings {
    fn default() -> Self {
        Self {
            target_fps: 60,
            cycles_per_frame: 10,
        }
    }
}

impl Default for RecordingSettings {
    fn default() -> Self {
        Self {
            gif_scale_factor: 8,
            gif_frame_delay: 4,
            gif_frame_skip: 3,
            output_dir: ".".to_string(),
        }
    }
}

impl Settings {
    /// Load settings from a TOML file, falling back to defaults if the file doesn't exist
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            let settings: Settings = toml::from_str(&contents)?;
            Ok(settings)
        } else {
            // Return default settings if file doesn't exist
            Ok(Settings::default())
        }
    }
    
    /// Save current settings to a TOML file
    #[allow(dead_code)]
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(path, toml_string)?;
        Ok(())
    }
}

