use std::ffi::{c_void, c_char, CStr};

use crate::{AudioSystem, AudioBuffer, stream::{Wav, AudioStream}};

#[repr(C)]
pub enum AudioResult {
    Ok,

    InvalidBuffer,
    InvalidVoice,
    InvalidValue,
    InvalidOperation,

    Other
}

impl AudioResult {
    pub fn from_mixr(result: crate::MixrError) -> Self {
        match result.e_type {
            crate::ErrorType::InvalidBuffer => Self::InvalidBuffer,
            crate::ErrorType::InvalidVoice => Self::InvalidVoice,
            crate::ErrorType::InvalidValue => Self::InvalidValue,
            crate::ErrorType::InvalidOperation => Self::InvalidOperation,
            crate::ErrorType::Other => Self::Other,
        }
    }
}

#[repr(C)]
pub enum DataType {
    I8,
    U8,
    I16,
    U16,
    I32,
    F32,
    F64
}

impl DataType {
    pub fn to_mixr(&self) -> crate::DataType {
        match self {
            DataType::I8 => crate::DataType::I8,
            DataType::U8 => crate::DataType::U8,
            DataType::I16 => crate::DataType::I16,
            DataType::U16 => crate::DataType::U16,
            DataType::I32 => crate::DataType::I32,
            DataType::F32 => crate::DataType::F32,
            DataType::F64 => crate::DataType::F64,
        }
    }
}

#[repr(C)]
pub struct AudioFormat {
    pub data_type: DataType,
    pub sample_rate: u32,
    pub channels: u8
}

impl AudioFormat {
    pub fn to_mixr(&self) -> crate::AudioFormat {
        crate::AudioFormat {
            data_type: self.data_type.to_mixr(),
            sample_rate: self.sample_rate,
            channels: self.channels
        }
    }
}

#[repr(C)]
pub struct BufferDescription {
    pub format: AudioFormat
}

impl BufferDescription {
    pub fn to_mixr(&self) -> crate::BufferDescription {
        crate::BufferDescription {
            format: self.format.to_mixr()
        }
    }
}

#[repr(C)]
pub struct PlayProperties {
    pub volume: f64,
    pub speed: f64,
    pub panning: f64,
    pub looping: bool,
    pub loop_start: usize,
    pub loop_end: usize,
    pub start_sample: usize
}

impl PlayProperties {
    pub fn to_mixr(&self) -> crate::PlayProperties {
        crate::PlayProperties {
            volume: self.volume,
            speed: self.speed,
            panning: self.panning,
            looping: self.looping,
            loop_start: self.loop_start,
            loop_end: self.loop_end,
            start_sample: self.start_sample
        }
    }
}

type Stream = *mut c_void;

#[no_mangle]
pub unsafe extern fn mxCreateSystem(sample_rate: u32, voices: u16) -> *mut AudioSystem {
    let system = AudioSystem::new(sample_rate, voices);

    Box::into_raw(Box::new(system))
}

#[no_mangle]
pub unsafe extern fn mxDestroySystem(system: &mut AudioSystem) {
    std::mem::drop(system)
}

#[no_mangle]
pub unsafe extern fn mxCreateBuffer(system: &mut AudioSystem, description: BufferDescription, data: *const c_void, length: usize, buffer: *mut AudioBuffer) -> AudioResult {
    // If data is nullptr, that is equivalent to there being no data. mixr itself cannot handle nullpts, so we must use `None` instead.
    let data = if data == std::ptr::null() {
        None
    } else {
        Some(std::slice::from_raw_parts(data as *const u8, length))
    };

    let buf = system.create_buffer(description.to_mixr(), data);

    let buf = match buf {
        Ok(buf) => buf,
        Err(err) => return AudioResult::from_mixr(err),
    };

    *buffer = buf;
    AudioResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxDestroyBuffer(system: &mut AudioSystem, buffer: AudioBuffer) -> AudioResult {
    let result = system.destroy_buffer(buffer);

    match result {
        Ok(_) => AudioResult::Ok,
        Err(err) => AudioResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxPlayBuffer(system: &mut AudioSystem, buffer: AudioBuffer, voice: u16, properties: PlayProperties) -> AudioResult {
    let result = system.play_buffer(buffer, voice, properties.to_mixr());

    match result {
        Ok(_) => AudioResult::Ok,
        Err(err) => AudioResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxReadBufferStereoF32(system: &mut AudioSystem, buffer: *mut f32, length: usize) {
    let slice = std::slice::from_raw_parts_mut(buffer, length);

    system.read_buffer_stereo_f32(slice);
}

#[no_mangle]
pub unsafe extern fn mxStreamLoadWav(path: *const c_char) -> Stream {
    let wav = Wav::from_file(CStr::from_ptr(path).to_str().unwrap()).unwrap();

    Box::into_raw(Box::new(wav)) as *mut _
}

#[no_mangle]
pub unsafe extern fn mxStreamFree(stream: Stream) {
    std::mem::drop(Box::from_raw(stream));
}

#[no_mangle]
pub unsafe extern fn mxStreamWavGetPcm(stream: Stream, data: *mut *mut c_void, length: *mut usize) {
    let stream: Box<Wav> = Box::from_raw(stream as *mut _);

    let pcm = stream.get_pcm()
}