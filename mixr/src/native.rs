use std::ffi::{c_void, c_char, CStr};

use crate::{stream::{AudioStream, Stream, Wav}, AudioCallbacks};
use crate::stream::Vorbis;

pub struct MxAudioSystem {
    system: crate::AudioSystem
}

#[repr(C)]
pub struct MxAudioBuffer {
    id: usize
}

#[repr(C)]
pub enum MxResult {
    Ok,

    InvalidBuffer,
    InvalidVoice,
    InvalidValue,
    InvalidOperation,

    FileNotFound,

    Other
}

impl MxResult {
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
pub enum MxDataType {
    I8,
    U8,
    I16,
    U16,
    I32,
    F32,
    F64
}

impl MxDataType {
    pub fn to_mixr(&self) -> crate::DataType {
        match self {
            MxDataType::I8 => crate::DataType::I8,
            MxDataType::U8 => crate::DataType::U8,
            MxDataType::I16 => crate::DataType::I16,
            MxDataType::U16 => crate::DataType::U16,
            MxDataType::I32 => crate::DataType::I32,
            MxDataType::F32 => crate::DataType::F32,
            MxDataType::F64 => crate::DataType::F64,
        }
    }

    pub fn from_mixr(data_type: crate::DataType) -> Self {
        match data_type {
            crate::DataType::I8 =>  MxDataType::I8,
            crate::DataType::U8 =>  MxDataType::U8,
            crate::DataType::I16 => MxDataType::I16,
            crate::DataType::U16 => MxDataType::U16,
            crate::DataType::I32 => MxDataType::I32,
            crate::DataType::F32 => MxDataType::F32,
            crate::DataType::F64 => MxDataType::F64,
        }
    }
}

#[repr(C)]
pub struct MxAudioFormat {
    pub data_type: MxDataType,
    pub sample_rate: u32,
    pub channels: u8
}

impl MxAudioFormat {
    pub fn to_mixr(&self) -> crate::AudioFormat {
        crate::AudioFormat {
            data_type: self.data_type.to_mixr(),
            sample_rate: self.sample_rate,
            channels: self.channels
        }
    }

    pub fn from_mixr(format: crate::AudioFormat) -> Self {
        Self {
            data_type: MxDataType::from_mixr(format.data_type),
            sample_rate: format.sample_rate,
            channels: format.channels
        }
    }
}

#[repr(C)]
pub struct MxBufferDescription {
    pub format: MxAudioFormat
}

impl MxBufferDescription {
    pub fn to_mixr(&self) -> crate::BufferDescription {
        crate::BufferDescription {
            format: self.format.to_mixr()
        }
    }
}

#[repr(C)]
pub struct MxPlayProperties {
    pub volume: f64,
    pub speed: f64,
    pub panning: f64,
    pub looping: bool,
    pub loop_start: usize,
    pub loop_end: usize,
    pub start_sample: usize
}

impl MxPlayProperties {
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
pub enum MxPlayState {
    Stopped,
    Paused,
    Playing
}

impl MxPlayState {
    pub fn to_mixr(&self) -> crate::PlayState {
        match self {
            MxPlayState::Stopped => crate::PlayState::Stopped,
            MxPlayState::Paused => crate::PlayState::Paused,
            MxPlayState::Playing => crate::PlayState::Playing,
        }
    }

    pub fn from_mixr(state: crate::PlayState) -> Self {
        match state {
            crate::PlayState::Stopped => Self::Stopped,
            crate::PlayState::Paused => Self::Paused,
            crate::PlayState::Playing => MxPlayState::Playing,
        }
    }
}

pub struct MxStream {
    stream: Box<dyn AudioStream>
}

#[no_mangle]
pub unsafe extern fn mxCreateSystem(sample_rate: u32, voices: u16) -> *mut MxAudioSystem {
    let system = crate::AudioSystem::new(sample_rate, voices);
    let system = MxAudioSystem { system };

    Box::into_raw(Box::new(system))
}

#[no_mangle]
pub unsafe extern fn mxDestroySystem(system: *mut MxAudioSystem) {
    let system = Box::from_raw(system);
    std::mem::drop(system)
}

#[no_mangle]
pub unsafe extern fn mxCreateBuffer(system: &mut MxAudioSystem, description: MxBufferDescription, data: *const c_void, length: usize, buffer: *mut MxAudioBuffer) -> MxResult {
    // If data is nullptr, that is equivalent to there being no data. mixr itself cannot handle nullpts, so we must use `None` instead.
    let data = if data == std::ptr::null() {
        None
    } else {
        Some(std::slice::from_raw_parts(data as *const u8, length))
    };

    let buf = system.system.create_buffer(description.to_mixr(), data);

    let buf = match buf {
        Ok(buf) => buf,
        Err(err) => return MxResult::from_mixr(err),
    };

    *buffer = MxAudioBuffer { id: buf.id };
    MxResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxDestroyBuffer(system: &mut MxAudioSystem, buffer: MxAudioBuffer) -> MxResult {
    let result = system.system.destroy_buffer(crate::AudioBuffer { id: buffer.id });

    match result {
        Ok(_) => MxResult::Ok,
        Err(err) => MxResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxUpdateBuffer(system: &mut MxAudioSystem, buffer: MxAudioBuffer, format: MxAudioFormat, data: *const c_void, length: usize) -> MxResult {
    let data = if data == std::ptr::null() {
        None
    } else {
        Some(std::slice::from_raw_parts(data as *const u8, length))
    };

    let result = system.system.update_buffer(crate::AudioBuffer { id: buffer.id }, format.to_mixr(), data);

    match result {
        Ok(_) => MxResult::Ok,
        Err(err) => MxResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxPlayBuffer(system: &mut MxAudioSystem, buffer: MxAudioBuffer, voice: u16, properties: MxPlayProperties) -> MxResult {
    let result = system.system.play_buffer(crate::AudioBuffer { id: buffer.id }, voice, properties.to_mixr());

    match result {
        Ok(_) => MxResult::Ok,
        Err(err) => MxResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxQueueBuffer(system: &mut MxAudioSystem, buffer: MxAudioBuffer, voice: u16) -> MxResult {
    let result = system.system.queue_buffer(crate::AudioBuffer { id: buffer.id }, voice);

    match result {
        Ok(_) => MxResult::Ok,
        Err(err) => MxResult::from_mixr(err)
    }
}

#[no_mangle]
pub unsafe extern fn mxGetPlayProperties(system: &mut MxAudioSystem, voice: u16, properties: *mut MxPlayProperties) -> MxResult {
    let props = system.system.get_play_properties(voice);

    let props = match props {
        Ok(props) => MxPlayProperties::from_mixr(props),
        Err(err) => return MxResult::from_mixr(err),
    };

    *properties = props;
    MxResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxSetPlayProperties(system: &mut MxAudioSystem, voice: u16, properties: MxPlayProperties) -> MxResult {
    let result = system.system.set_play_properties(voice, properties.to_mixr());

    match result {
        Ok(_) => MxResult::Ok,
        Err(err) => MxResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxGetVoiceState(system: &mut MxAudioSystem, voice: u16, state: *mut MxPlayState) -> MxResult {
    let mx_state = system.system.get_voice_state(voice);

    let mx_state = match mx_state {
        Ok(state) => MxPlayState::from_mixr(state),
        Err(err) => return MxResult::from_mixr(err),
    };

    *state = mx_state;
    MxResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxSetVoiceState(system: &mut MxAudioSystem, voice: u16, state: MxPlayState) -> MxResult {
    let result = system.system.set_voice_state(voice, state.to_mixr());

    match result {
        Ok(_) => MxResult::Ok,
        Err(err) => MxResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxGetPositionSamples(system: &mut MxAudioSystem, voice: u16, position: *mut usize) -> MxResult {
    let pos = system.system.get_position_samples(voice);

    let pos = match pos {
        Ok(pos) => pos,
        Err(err) => return MxResult::from_mixr(err),
    };

    *position = pos;
    MxResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxSetPositionSamples(system: &mut MxAudioSystem, voice: u16, position: usize) -> MxResult {
    let result = system.system.set_position_samples(voice, position);

    match result {
        Ok(_) => MxResult::Ok,
        Err(err) => MxResult::from_mixr(err)
    }
}

#[no_mangle]
pub unsafe extern fn mxGetPosition(system: &mut MxAudioSystem, voice: u16, position: *mut f64) -> MxResult {
    let pos = system.system.get_position(voice);

    let pos = match pos {
        Ok(pos) => pos,
        Err(err) => return MxResult::from_mixr(err),
    };

    *position = pos;
    MxResult::Ok
}

#[no_mangle]
pub unsafe extern fn mxSetPosition(system: &mut MxAudioSystem, voice: u16, position: f64) -> MxResult {
    let result = system.system.set_position(voice, position);

    match result {
        Ok(_) => MxResult::Ok,
        Err(err) => MxResult::from_mixr(err),
    }
}

#[no_mangle]
pub unsafe extern fn mxSetBufferFinishedCallback(system: &mut MxAudioSystem, callback: extern fn(MxAudioBuffer, u16)) {
    let cb = Callback { callback };

    system.system.set_callback(Box::new(cb));
}

#[no_mangle]
pub unsafe extern fn mxReadBufferStereoF32(system: &mut MxAudioSystem, buffer: *mut f32, length: usize) {
    let slice = std::slice::from_raw_parts_mut(buffer, length);

    system.system.read_buffer_stereo_f32(slice);
}

#[no_mangle]
pub unsafe extern fn mxGetNumVoices(system: &mut MxAudioSystem) -> u16 {
    system.system.num_voices()
}

#[no_mangle]
pub unsafe extern fn mxStreamLoadFile(path: *const c_char, stream: *mut *mut MxStream) -> MxResult {
    load_stream_file::<Stream>(path, stream)
}

#[no_mangle]
pub unsafe extern fn mxStreamLoadWavFile(path: *const c_char, stream: *mut *mut MxStream) -> MxResult {
    load_stream_file::<Wav>(path, stream)
}

#[no_mangle]
pub unsafe extern fn mxStreamLoadVorbisFile(path: *const c_char, stream: *mut *mut MxStream) -> MxResult {
    load_stream_file::<Vorbis>(path, stream)
}

#[no_mangle]
pub unsafe extern fn mxStreamFree(stream: &mut MxStream) {
    std::mem::drop(Box::from_raw(stream));
}

#[no_mangle]
pub unsafe extern fn mxStreamGetFormat(stream: &mut MxStream, format: *mut MxAudioFormat) {
    let stream = &stream.stream;
    let fmt = MxAudioFormat::from_mixr(stream.format());

    *format = fmt;
}

#[no_mangle]
pub unsafe extern fn mxStreamGetBuffer(stream: &mut MxStream, buffer: *mut u8, length: usize) -> usize {
    let stream = &mut stream.stream;

    let mut slice = std::slice::from_raw_parts_mut(buffer, length);

    stream.get_buffer(&mut slice).unwrap()
}

#[no_mangle]
pub unsafe extern fn mxStreamGetPcm(stream: &mut MxStream, data: *mut c_void, length: *mut usize) {
    let stream = &mut stream.stream;

    *length = stream.pcm_length();

    // Only attempt to set the stream data if the pointer is not null.
    if data != std::ptr::null_mut() {
        // To avoid memory allocations, we just call get_buffer on the full pointer.
        stream.get_buffer(std::slice::from_raw_parts_mut(data as *mut _, *length)).unwrap();
    }
}

#[no_mangle]
pub unsafe extern fn mxStreamSeek(stream: &mut MxStream, position: f64) {
    let stream = &mut stream.stream;

    stream.seek(position);
}

#[no_mangle]
pub unsafe extern fn mxStreamSeekSamples(stream: &mut MxStream, position: usize) {
    let stream = &mut stream.stream;

    stream.seek_samples(position);
}

#[no_mangle]
pub unsafe extern fn mxStreamRestart(stream: &mut MxStream) {
    let stream = &mut stream.stream;

    stream.restart();
}

struct Callback {
    callback: unsafe extern fn(MxAudioBuffer, u16)
}

impl AudioCallbacks for Callback {
    fn buffer_finished(&mut self, buffer: crate::AudioBuffer, voice: u16) {
        unsafe {
            (self.callback)(MxAudioBuffer { id: buffer.id }, voice);
        }
    }
}

unsafe fn load_stream_file<T: AudioStream + 'static>(path: *const c_char, stream: *mut *mut MxStream) -> MxResult {
    let a_stream = T::from_file(CStr::from_ptr(path).to_str().unwrap());

    let a_stream = match a_stream {
        Ok(a_stream) => Box::new(a_stream) as Box<dyn AudioStream>,
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                return MxResult::FileNotFound
            } else {
                return MxResult::Other
            }
        }
    };

    let mx_stream = MxStream {
        stream: a_stream
    };

    *stream = Box::into_raw(Box::new(mx_stream));

    MxResult::Ok
}