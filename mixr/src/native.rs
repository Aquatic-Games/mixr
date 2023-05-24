use std::ffi::{c_void, c_char, CStr};

use crate::{AudioSystem, AudioBuffer, stream::{Wav, AudioStream}};

#[repr(C)]
pub enum MixrResult {
    Ok,

    InvalidBuffer,
    InvalidVoice,
    InvalidValue,
    InvalidOperation,

    FileNotFound,

    Other
}

impl MixrResult {
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

    pub fn from_mixr(data_type: crate::DataType) -> Self {
        match data_type {
            crate::DataType::I8 =>  DataType::I8,
            crate::DataType::U8 =>  DataType::U8,
            crate::DataType::I16 => DataType::I16,
            crate::DataType::U16 => DataType::U16,
            crate::DataType::I32 => DataType::I32,
            crate::DataType::F32 => DataType::F32,
            crate::DataType::F64 => DataType::F64,
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

    pub fn from_mixr(format: crate::AudioFormat) -> Self {
        Self {
            data_type: DataType::from_mixr(format.data_type),
            sample_rate: format.sample_rate,
            channels: format.channels
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

    pub fn from_mixr(properties: crate::PlayProperties) -> Self {
        Self {
            volume: properties.volume,
            speed: properties.speed,
            panning: properties.panning,
            looping: properties.looping,
            loop_start: properties.loop_start,
            loop_end: properties.loop_end,
            start_sample: properties.start_sample
        }
    }
}

#[repr(C)]
pub enum PlayState {
    Stopped,
    Paused,
    Playing
}

impl PlayState {
    pub fn to_mixr(&self) -> crate::PlayState {
        match self {
            PlayState::Stopped => crate::PlayState::Stopped,
            PlayState::Paused => crate::PlayState::Paused,
            PlayState::Playing => crate::PlayState::Playing,
        }
    }

    pub fn from_mixr(state: crate::PlayState) -> Self {
        match state {
            crate::PlayState::Stopped => Self::Stopped,
            crate::PlayState::Paused => Self::Paused,
            crate::PlayState::Playing => PlayState::Playing,
        }
    }
}

pub struct Stream {
    stream: Box<dyn AudioStream>
}

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
pub unsafe extern fn mxCreateBuffer(system: &mut AudioSystem, description: BufferDescription, data: *const c_void, length: usize, buffer: *mut AudioBuffer) -> MixrResult {
    // If data is nullptr, that is equivalent to there being no data. mixr itself cannot handle nullpts, so we must use `None` instead.
    let data = if data == std::ptr::null() {
        None
    } else {
        Some(std::slice::from_raw_parts(data as *const u8, length))
    };

    let buf = system.create_buffer(description.to_mixr(), data);

    let buf = match buf {
        Ok(buf) => buf,
        Err(err) => return MixrResult::from_mixr(err),
    };

    *buffer = buf;
    MixrResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxDestroyBuffer(system: &mut AudioSystem, buffer: AudioBuffer) -> MixrResult {
    let result = system.destroy_buffer(buffer);

    match result {
        Ok(_) => MixrResult::Ok,
        Err(err) => MixrResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxUpdateBuffer(system: &mut AudioSystem, buffer: AudioBuffer, format: AudioFormat, data: *const c_void, length: usize) -> MixrResult {
    let data = if data == std::ptr::null() {
        None
    } else {
        Some(std::slice::from_raw_parts(data as *const u8, length))
    };

    let result = system.update_buffer(buffer, format.to_mixr(), data);

    match result {
        Ok(_) => MixrResult::Ok,
        Err(err) => MixrResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxPlayBuffer(system: &mut AudioSystem, buffer: AudioBuffer, voice: u16, properties: PlayProperties) -> MixrResult {
    let result = system.play_buffer(buffer, voice, properties.to_mixr());

    match result {
        Ok(_) => MixrResult::Ok,
        Err(err) => MixrResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxGetPlayProperties(system: &mut AudioSystem, voice: u16, properties: *mut PlayProperties) -> MixrResult {
    let props = system.get_play_properties(voice);

    let props = match props {
        Ok(props) => PlayProperties::from_mixr(props),
        Err(err) => return MixrResult::from_mixr(err),
    };

    *properties = props;
    MixrResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxSetPlayProperties(system: &mut AudioSystem, voice: u16, properties: PlayProperties) -> MixrResult {
    let result = system.set_play_properties(voice, properties.to_mixr());

    match result {
        Ok(_) => MixrResult::Ok,
        Err(err) => MixrResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxGetVoiceState(system: &mut AudioSystem, voice: u16, state: *mut PlayState) -> MixrResult {
    let mx_state = system.get_voice_state(voice);

    let mx_state = match mx_state {
        Ok(state) => PlayState::from_mixr(state),
        Err(err) => return MixrResult::from_mixr(err),
    };

    *state = mx_state;
    MixrResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxSetVoiceState(system: &mut AudioSystem, voice: u16, state: PlayState) -> MixrResult {
    let result = system.set_voice_state(voice, state.to_mixr());

    match result {
        Ok(_) => MixrResult::Ok,
        Err(err) => MixrResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxGetPositionSamples(system: &mut AudioSystem, voice: u16, position: *mut usize) -> MixrResult {
    let pos = system.get_position_samples(voice);

    let pos = match pos {
        Ok(pos) => pos,
        Err(err) => return MixrResult::from_mixr(err),
    };

    *position = pos;
    MixrResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxSetPositionSamples(system: &mut AudioSystem, voice: u16, position: usize) -> MixrResult {
    let result = system.set_position_samples(voice, position);

    match result {
        Ok(_) => MixrResult::Ok,
        Err(err) => MixrResult::from_mixr(err)
    }
}

#[no_mangle]
pub unsafe extern fn mxGetPosition(system: &mut AudioSystem, voice: u16, position: *mut f64) -> MixrResult {
    let pos = system.get_position(voice);

    let pos = match pos {
        Ok(pos) => pos,
        Err(err) => return MixrResult::from_mixr(err),
    };

    *position = pos;
    MixrResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxSetPosition(system: &mut AudioSystem, voice: u16, position: f64) -> MixrResult {
    let result = system.set_position(voice, position);

    match result {
        Ok(_) => MixrResult::Ok,
        Err(err) => MixrResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxReadBufferStereoF32(system: &mut AudioSystem, buffer: *mut f32, length: usize) {
    let slice = std::slice::from_raw_parts_mut(buffer, length);

    system.read_buffer_stereo_f32(slice);
}

#[no_mangle]
pub unsafe extern fn mxStreamLoadWav(path: *const c_char, stream: *mut *mut Stream) -> MixrResult {
    let wav = Wav::from_file(CStr::from_ptr(path).to_str().unwrap());

    let wav = match wav {
        Ok(wav) => wav,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                return MixrResult::FileNotFound
            } else {
                return MixrResult::Other
            }
        }
    };

    let mx_stream = Stream {
        stream: Box::new(wav)
    };

    *stream = Box::into_raw(Box::new(mx_stream));

    MixrResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxStreamFree(stream: &mut Stream) {
    std::mem::drop(Box::from_raw(stream));
}

#[no_mangle]
pub unsafe extern fn mxStreamGetFormat(stream: &mut Stream, format: *mut AudioFormat) {
    let stream = &stream.stream;
    let fmt = AudioFormat::from_mixr(stream.format());

    *format = fmt;
}

#[no_mangle]
pub unsafe extern fn mxStreamGetPcm(stream: &mut Stream, data: *mut c_void, length: *mut usize) {
    let stream = &mut stream.stream;

    *length = stream.pcm_length();

    // Only attempt to set the stream data if the pointer is not null.
    if data != std::ptr::null_mut() {
        // To avoid memory allocations, we just call get_buffer on the full pointer.
        stream.get_buffer(std::slice::from_raw_parts_mut(data as *mut _, *length)).unwrap();
    }
}