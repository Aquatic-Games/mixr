pub mod system;
pub mod binary_reader;

//#[cfg(feature = "cmixr")]
mod cmixr;

//#[cfg(feature = "loaders")]
pub mod loaders;

//#[cfg(feature = "engine")]
pub mod engine;

#[derive(Clone, Debug, Copy, PartialEq)]
#[repr(C)]
pub enum FormatType {
    U8,
    I8,
    U16,
    I16,
    I32,
    F32,
    F64,
}

#[derive(Clone, Debug, Copy, PartialEq)]
#[repr(C)]
pub struct AudioFormat {
    pub channels:    u8,
    pub sample_rate: i32,
    pub format_type: FormatType
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self {
            channels: 2,
            sample_rate: 48000,
            format_type: FormatType::I16
        }
    }
}

impl AudioFormat {
    pub fn bits_per_sample(&self) -> u8 {
        match self.format_type {
            FormatType::U8 => 8,
            FormatType::I8 => 8,
            FormatType::U16 => 16,
            FormatType::I16 => 16,
            FormatType::I32 => 32,
            FormatType::F32 => 32,
            FormatType::F64 => 64,
        }
    }

    pub fn bytes_per_sample(&self) -> u8 {
        self.bits_per_sample() / 8
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
    OutOfRange
}

pub trait ByteConvert<T> {
    fn from_bytes_le(bytes: &[u8]) -> T;
}

impl ByteConvert<f32> for f32 {
    #[inline(always)]
    fn from_bytes_le(bytes: &[u8]) -> f32 {
        unsafe { std::mem::transmute::<i32, f32>(i32::from_bytes_le(bytes)) }
    }
}

impl ByteConvert<f64> for f64 {
    #[inline(always)]
    fn from_bytes_le(bytes: &[u8]) -> f64 {
        unsafe { std::mem::transmute::<i64, f64>(i64::from_bytes_le(bytes)) }
    }
}

impl ByteConvert<i32> for i32 {
    #[inline(always)]
    fn from_bytes_le(bytes: &[u8]) -> i32 {
        bytes[0] as i32 | ((bytes[1] as i32) << 8) | ((bytes[2] as i32) << 16)| ((bytes[3] as i32) << 24)
    }
}

impl ByteConvert<i64> for i64 {
    #[inline(always)]
    fn from_bytes_le(bytes: &[u8]) -> i64 {
        bytes[0] as i64 | ((bytes[1] as i64) << 8) | ((bytes[2] as i64) << 16)| ((bytes[3] as i64) << 24) | 
        ((bytes[4] as i64) << 32) | ((bytes[5] as i64) << 40) | ((bytes[6] as i64) << 48) | ((bytes[7] as i64) << 56)
    }
}