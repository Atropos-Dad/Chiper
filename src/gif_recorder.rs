use gif::{Encoder, Frame, Repeat};
use std::fs::File;
use std::io::BufWriter;
use std::thread;
use crossbeam_channel::{Sender, Receiver, bounded};
use crate::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT};
use crate::settings::RecordingSettings;
use std::sync::Arc;

// Buffer size constant that doesn't change
const RECORDING_BUFFER_SIZE: usize = 30; // Buffer up to 30 frames

pub struct GifRecorder {
    sender: Option<Sender<RecordCommand>>,
    thread_handle: Option<thread::JoinHandle<()>>,
    frame_count: u32,
    settings: Arc<RecordingSettings>,
}

enum RecordCommand {
    AddFrame(Vec<u8>),
    Stop,
}

impl GifRecorder {
    pub fn new() -> Self {
        Self::with_settings(Arc::new(RecordingSettings::default()))
    }
    
    pub fn with_settings(settings: Arc<RecordingSettings>) -> Self {
        Self {
            sender: None,
            thread_handle: None,
            frame_count: 0,
            settings,
        }
    }

    pub fn generate_filename(rom_name: &str, output_dir: &str, pattern: &str) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let filename = pattern
            .replace("{rom_name}", rom_name)
            .replace("{timestamp}", &timestamp.to_string());
        
        format!("{}/{}.gif", output_dir, filename)
    }

    pub fn start_recording(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_recording() {
            return Err("Already recording".into());
        }

        let (sender, receiver) = bounded(RECORDING_BUFFER_SIZE); // Buffer up to 30 frames
        self.sender = Some(sender);
        self.frame_count = 0;

        let filename = filename.to_string();
        let settings = self.settings.clone();
        let thread_handle = thread::spawn(move || {
            if let Err(e) = recording_thread(receiver, filename, settings) {
                eprintln!("Recording thread error: {}", e);
            }
        });

        self.thread_handle = Some(thread_handle);
        println!("Started GIF recording");
        Ok(())
    }

    pub fn add_frame(&mut self, rgba_buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref sender) = self.sender {
            self.frame_count += 1;
            
            // Skip frames to reduce load
            if self.frame_count % (self.settings.gif_frame_skip + 1) != 0 {
                return Ok(());
            }

            // Clone the buffer for the recording thread
            let buffer = rgba_buffer.to_vec();
            
            // Try to send, but don't block if the buffer is full
            match sender.try_send(RecordCommand::AddFrame(buffer)) {
                Ok(_) => {},
                Err(crossbeam_channel::TrySendError::Full(_)) => {
                    // Skip this frame if buffer is full
                    println!("Recording buffer full, skipping frame");
                }
                Err(e) => return Err(Box::new(e)),
            }
        }
        Ok(())
    }

    pub fn stop_recording(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(sender) = self.sender.take() {
            sender.send(RecordCommand::Stop)?;
            
            if let Some(handle) = self.thread_handle.take() {
                handle.join().map_err(|_| "Recording thread panicked")?;
            }
            
            println!("GIF recording stopped and saved");
        }
        Ok(())
    }

    pub fn is_recording(&self) -> bool {
        self.sender.is_some()
    }

}

impl Drop for GifRecorder {
    fn drop(&mut self) {
        if self.is_recording() {
            let _ = self.stop_recording();
        }
    }
}

fn recording_thread(receiver: Receiver<RecordCommand>, filename: String, settings: Arc<RecordingSettings>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(&filename)?;
    let writer = BufWriter::new(file);
    
    let scale_factor = settings.gif_scale_factor;
    let scaled_width = (DISPLAY_WIDTH * scale_factor as usize) as u16;
    let scaled_height = (DISPLAY_HEIGHT * scale_factor as usize) as u16;
    
    let mut encoder = Encoder::new(writer, scaled_width, scaled_height, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;
    
    let frame_delay = settings.gif_frame_delay; // Delay in centiseconds

    loop {
        match receiver.recv()? {
            RecordCommand::AddFrame(rgba_buffer) => {
                // Convert and scale in the background thread
                let scaled_rgb = scale_and_convert_buffer(&rgba_buffer, scale_factor);
                
                let mut frame = Frame::from_rgb(scaled_width, scaled_height, &scaled_rgb);
                frame.delay = frame_delay;
                
                encoder.write_frame(&frame)?;
            }
            RecordCommand::Stop => break,
        }
    }

    drop(encoder); // Finalize the GIF
    println!("GIF saved: {}", filename);
    Ok(())
}

fn scale_and_convert_buffer(rgba_buffer: &[u8], scale_factor: u16) -> Vec<u8> {
    let scaled_width = DISPLAY_WIDTH * scale_factor as usize;
    let scaled_height = DISPLAY_HEIGHT * scale_factor as usize;
    let mut scaled_rgb = vec![0u8; scaled_width * scaled_height * 3];

    // Use a more efficient scaling approach
    for y in 0..DISPLAY_HEIGHT {
        for x in 0..DISPLAY_WIDTH {
            let src_idx = (y * DISPLAY_WIDTH + x) * 4;
            let r = rgba_buffer[src_idx];
            let g = rgba_buffer[src_idx + 1];
            let b = rgba_buffer[src_idx + 2];

            // Calculate the starting position for this pixel in the scaled buffer
            let scaled_y = y * scale_factor as usize;
            let scaled_x = x * scale_factor as usize;

            // Fill the scaled area with the same color
            for sy in 0..scale_factor as usize {
                let dst_y = scaled_y + sy;
                let row_start = dst_y * scaled_width * 3;
                
                for sx in 0..scale_factor as usize {
                    let dst_x = scaled_x + sx;
                    let dst_idx = row_start + dst_x * 3;
                    
                    scaled_rgb[dst_idx] = r;
                    scaled_rgb[dst_idx + 1] = g;
                    scaled_rgb[dst_idx + 2] = b;
                }
            }
        }
    }

    scaled_rgb
}