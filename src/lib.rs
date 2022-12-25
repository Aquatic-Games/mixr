pub mod system;
pub mod loaders;
pub mod binary_reader;
mod cmixr;

#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub struct AudioFormat {
    pub channels: u8,
    pub sample_rate: i32,
    pub bits_per_sample: u8
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self {
            channels: 2,
            sample_rate: 48000,
            bits_per_sample: 16
        }
    }
}

#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub enum InterpolationType {
    None,
    Linear
}

#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub struct ChannelProperties {
    pub volume: f64,
    pub speed: f64,
    pub panning: f64,
    pub looping: bool,
    pub interpolation_type: InterpolationType,

    pub loop_start: i32,
    pub loop_end: i32,
}

impl Default for ChannelProperties {
    fn default() -> Self {
        Self { volume: 1.0, speed: 1.0, panning: 0.5, looping: false, interpolation_type: InterpolationType::Linear, loop_start: 0, loop_end: -1 }
    }
}

#[repr(C)]
pub enum ChannelState {
    Playing,
    Stopped
}

#[repr(C)]
pub enum AudioResult {
    Ok,

    InvalidBuffer,
    InvalidChannel,
    NoChannels
}