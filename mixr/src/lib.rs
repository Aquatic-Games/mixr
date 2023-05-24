pub mod stream;
pub mod engine;
pub mod native;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorType {
    InvalidBuffer,
    InvalidVoice,
    InvalidValue,
    InvalidOperation,

    Other
}

#[derive(Debug, Clone, Copy)]
pub struct MixrError<'a> {
    pub e_type:  ErrorType,
    pub message: &'a str
}

pub type MixrResult<'a, TResult> = Result<TResult, MixrError<'a>>;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn bits(&self) -> u8 {
        match self {
            DataType::I8 => 8,
            DataType::U8 => 8,
            DataType::I16 => 16,
            DataType::U16 => 16,
            DataType::I32 => 32,
            DataType::F32 => 32,
            DataType::F64 => 64,
        }
    }

    pub fn bytes(&self) -> u8 {
        self.bits() / 8
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioFormat {
    pub data_type: DataType,
    pub sample_rate: u32,
    pub channels: u8
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self { data_type: DataType::F32, sample_rate: 480000, channels: 2 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BufferDescription {
    pub format: AudioFormat
}

/// Various properties that denote how mixr will play back a sound.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayProperties {
    /// The relative volume to play the sound at.
    /// A value of 1.0 means no change in the volume.
    /// A value of 0.0 means the sound will play, but will not be audiable.
    /// Values outside of this range are supported but may produce less than ideal results.
    pub volume: f64,

    /// The relative speed to play the sound at.
    /// A speed of 1.0 means the sound will play at its original speed.
    /// A speed of 2.0 means the sound will play at double its original speed.
    /// For best results, use values in the range of 0.5 to 2.0, however values outside this range are supported.
    pub speed: f64,

    /// The Left-Right panning of the sound.
    /// A value of 0 means central panning. A value of -1 means pan fully to the left, and a value of 1 means pan fully to the right.
    /// Values outside of this range are invalid and will return an [`Err()`].
    pub panning: f64,
    
    /// If enabled, the sample will loop between `loop_start` and `loop_end`.
    pub looping: bool,

    /// The starting point, in samples, of the loop. Once `loop_end` is reached, the playback position will jump to here.
    pub loop_start: usize,

    /// The ending point, in samples, of the loop. Once the playback position reaches this value, it will jump back to `loop_start`.
    /// Setting this field to a value of 0 will tell mixr you want to loop at the end of the sound.
    pub loop_end: usize,

    /// The starting point, in samples, at which to start playing the sound. All samples before this point will not be played.
    pub start_sample: usize
}

impl Default for PlayProperties {
    fn default() -> Self {
        Self { volume: 1.0, speed: 1.0, panning: 0.0, looping: false, loop_start: 0, loop_end: 0, start_sample: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayState {
    Stopped,
    Paused,
    Playing
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct AudioBuffer {
    id: usize
}

pub trait AudioCallbacks {
    fn buffer_finished(&mut self, buffer: AudioBuffer, voice: u16);
}

pub struct AudioSystem {
    pub volume: f64,
    //pub fade_in: usize,

    sample_rate: u32,

    buffers: Vec<Option<Buffer>>,
    voices: Vec<Voice>
}

impl AudioSystem {
    pub fn new(sample_rate: u32, voices: u16) -> Self {
        let mut channels = Vec::with_capacity(voices as usize);

        for _ in 0..voices {
            channels.push(Voice { 
                buffer: usize::MAX,
                is_playing: false,

                position: 0,
                float_pos: 0.0,

                speed: 0.0,

                properties: PlayProperties::default(),
                sample_end: 0,

                previous_position: 0,
                lerp_pos: 0,

                alignment: 0
            });
        }

        Self {
            volume: 1.0,
            //fade_in: 20,

            sample_rate,

            buffers: Vec::new(),
            voices: channels
        }
    }

    pub fn create_buffer<T>(&mut self, description: BufferDescription, data: Option<&[T]>) -> MixrResult<AudioBuffer> {
        if description.format.channels > 2 || description.format.channels < 1 {
            return Err(MixrError { e_type: ErrorType::InvalidValue, message: "Buffer format must have either 1 or 2 channels." });
        }

        let data: Vec<u8> = if let Some(dat) = data {
            let length = dat.len() * std::mem::size_of::<T>();
            unsafe { std::slice::from_raw_parts(dat.as_ptr() as *mut _, length).to_vec() }
        } else {
            Vec::new()
        };

        // Calculate byte alignment
        let alignment = (description.format.data_type.bytes() * description.format.channels) as usize;

        let buffer = Buffer {
            data,
            format: description.format,

            alignment
        };

        self.buffers.push(Some(buffer));

        Ok(AudioBuffer {
            id: self.buffers.len() - 1
        })
    }

    pub fn destroy_buffer(&mut self, buffer: AudioBuffer) -> MixrResult<()> {
        let buffer = self.buffers.get_mut(buffer.id).ok_or(MixrError { e_type: ErrorType::InvalidBuffer, message: "The given buffer is invalid." })?;
        *buffer = None;

        Ok(())
    }

    pub fn update_buffer<T>(&mut self, buffer: AudioBuffer, format: AudioFormat, data: Option<&[T]>) -> MixrResult<()> {
        let buffer = self.buffers.get_mut(buffer.id).ok_or(MixrError { e_type: ErrorType::InvalidBuffer, message: "The given buffer is invalid." })?;

        let buffer = if let Some(buf) = buffer {
            buf
        } else {
            return Err(MixrError { e_type: ErrorType::InvalidBuffer, message: "The given buffer is invalid." });
        };

        buffer.data = if let Some(dat) = data {
            let length = dat.len() * std::mem::size_of::<T>();
            unsafe { std::slice::from_raw_parts(dat.as_ptr() as *mut _, length).to_vec() }
        } else {
            Vec::new()
        };

        buffer.format = format;

        let alignment = (format.data_type.bytes() * format.channels) as usize;
        buffer.alignment = alignment;

        Ok(())
    }

    pub fn play_buffer(&mut self, buffer: AudioBuffer, voice: u16, properties: PlayProperties) -> MixrResult<()> {
        let mut voice = self.voices.get_mut(voice as usize).ok_or(MixrError { e_type: ErrorType::InvalidVoice, message: "Voice was out of range." })?;
        let internal_buffer = self.buffers[buffer.id].as_ref().ok_or(MixrError { e_type: ErrorType::InvalidBuffer, message: "Buffer was not valid. Has it been deleted?" })?;

        if properties.looping && properties.loop_end != 0 {
            if properties.loop_end - properties.loop_start == 0 {
                return Err(MixrError { e_type: ErrorType::InvalidValue, message: "Loop size cannot total 0. (loop start and loop end values are the same)" });
            } else if properties.loop_end < properties.loop_start {
                return Err(MixrError { e_type: ErrorType::InvalidValue, message: "Loop end position cannot be less than the loop start position." });
            }
        }

        if properties.panning < -1.0 || properties.panning > 1.0 {
            return Err(MixrError { e_type: ErrorType::InvalidValue, message: "Panning value must be between -1.0 and 1.0." });
        }

        voice.buffer = buffer.id;
        voice.position = properties.start_sample;
        voice.float_pos = 0.0;

        voice.previous_position = 0;
        voice.lerp_pos = 0;

        voice.is_playing = true;

        let align = f64::ceil((internal_buffer.format.sample_rate as f64 / self.sample_rate as f64) * (properties.speed));

        voice.speed = (internal_buffer.format.sample_rate as f64 / self.sample_rate as f64) * (properties.speed / align);

        voice.alignment = internal_buffer.alignment * align as usize;

        voice.properties = properties;

        voice.sample_end = if properties.looping {
            if properties.loop_end == 0 { internal_buffer.data.len() } else { properties.loop_end * internal_buffer.alignment }
        } else {
            internal_buffer.data.len()
        };

        Ok(())
    }

    pub fn get_play_properties(&self, voice: u16) -> MixrResult<PlayProperties> {
        let voice = self.voices.get(voice as usize).ok_or(MixrError { e_type: ErrorType::InvalidVoice, message: "Voice was out of range." })?;

        Ok(voice.properties)
    }

    pub fn set_play_properties(&mut self, voice: u16, properties: PlayProperties) -> MixrResult<()> {
        let voice = self.voices.get_mut(voice as usize).ok_or(MixrError { e_type: ErrorType::InvalidVoice, message: "Voice was out of range." })?;

        voice.properties = properties;

        Ok(())
    }

    pub fn get_voice_state(&self, voice: u16) -> MixrResult<PlayState> {
        let voice = self.voices.get(voice as usize).ok_or(MixrError { e_type: ErrorType::InvalidVoice, message: "Voice was out of range." })?;

        if voice.is_playing {
            Ok(PlayState::Playing)
        } else if voice.buffer == usize::MAX {
            Ok(PlayState::Stopped)
        } else {
            Ok(PlayState::Paused)
        }
    }

    pub fn set_voice_state(&mut self, voice: u16, state: PlayState) -> MixrResult<()> {
        let voice = self.voices.get_mut(voice as usize).ok_or(MixrError { e_type: ErrorType::InvalidVoice, message: "Voice was out of range." })?;

        match state {
            PlayState::Stopped => {
                voice.is_playing = false;
                voice.buffer = usize::MAX;
                voice.position = 0;
                voice.float_pos = 0.0;
            },
            PlayState::Paused => {
                if voice.buffer == usize::MAX {
                    return Err(MixrError { e_type: ErrorType::InvalidOperation, message: "Cannot pause a stopped voice." });
                }

                voice.is_playing = false;
            },
            PlayState::Playing => {
                if voice.buffer == usize::MAX {
                    return Err(MixrError { e_type: ErrorType::InvalidOperation, message: "Cannot play a stopped voice. Use AudioSystem::play_buffer() instead." });
                }

                voice.is_playing = true;
            },
        }

        Ok(())
    }

    pub fn get_position_samples(&self, voice: u16) -> MixrResult<usize> {
        let voice = self.voices.get(voice as usize).ok_or(MixrError { e_type: ErrorType::InvalidVoice, message: "Voice was out of range." })?;

        let buffer = if let Some(buf) = self.buffers.get(voice.buffer) {
            buf
        } else {
            return Ok(0);
        };

        // The buffer that a voice is attached to may not necessarily exist in the vec, so we must check for that too.
        let buffer = if let Some(buf) = buffer {
            buf
        } else {
            return Ok(0);
        };

        Ok(voice.position * (voice.alignment / buffer.alignment))
    }

    pub fn set_position_samples(&mut self, voice: u16, position: usize) -> MixrResult<()> {
        let voice = self.voices.get_mut(voice as usize).ok_or(MixrError { e_type: ErrorType::InvalidVoice, message: "Voice was out of range." })?;
        
        let buffer = if let Some(buf) = self.buffers.get(voice.buffer) {
            buf
        } else {
            return Err(MixrError { e_type: ErrorType::InvalidOperation, message: "Cannot set the position of a voice that is stopped." });
        };

        // The buffer that a voice is attached to may not necessarily exist in the vec, so we must check for that too.
        let buffer = if let Some(buf) = buffer {
            buf
        } else {
            return Err(MixrError { e_type: ErrorType::InvalidOperation, message: "Cannot set the position of a voice that is stopped." });
        };

        if position >= buffer.data.len() / buffer.alignment {
            return Err(MixrError { e_type: ErrorType::InvalidValue, message: "Position is outside of the buffer's size." });
        }

        voice.position = position;

        Ok(())
    }

    pub fn get_position(&self, voice: u16) -> MixrResult<f64> {
        let voice = self.voices.get(voice as usize).ok_or(MixrError { e_type: ErrorType::InvalidVoice, message: "Voice was out of range." })?;

        let buffer = if let Some(buf) = self.buffers.get(voice.buffer) {
            buf
        } else {
            return Ok(0.0);
        };

        // The buffer that a voice is attached to may not necessarily exist in the vec, so we must check for that too.
        let buffer = if let Some(buf) = buffer {
            buf
        } else {
            return Ok(0.0);
        };

        // In order to be speed accurate, we must also take the alignment into account.
        // As such, as must divide the voice alignment by the buffer alignment. The voice alignment will ALWAYS be a multiple of the buffer alignment,
        // and will NEVER be less than the buffer's alignment.
        let position = ((voice.position as f64 * (voice.alignment / buffer.alignment) as f64) + voice.float_pos) / buffer.format.sample_rate as f64;

        Ok(position)
    }

    pub fn set_position(&mut self, voice: u16, position: f64) -> MixrResult<()> {
        if position < 0.0 {
            return Err(MixrError { e_type: ErrorType::InvalidValue, message: "Position must be at least 0." });
        }

        let voice = self.voices.get_mut(voice as usize).ok_or(MixrError { e_type: ErrorType::InvalidVoice, message: "Voice was out of range." })?;
        
        let buffer = if let Some(buf) = self.buffers.get(voice.buffer) {
            buf
        } else {
            return Err(MixrError { e_type: ErrorType::InvalidOperation, message: "Cannot set the position of a voice that is stopped." });
        };

        // The buffer that a voice is attached to may not necessarily exist in the vec, so we must check for that too.
        let buffer = if let Some(buf) = buffer {
            buf
        } else {
            return Err(MixrError { e_type: ErrorType::InvalidOperation, message: "Cannot set the position of a voice that is stopped." });
        };

        let position = position * buffer.format.sample_rate as f64;
        let p_usize = position as usize;

        if p_usize >= buffer.data.len() / buffer.alignment {
            return Err(MixrError { e_type: ErrorType::InvalidValue, message: "Position is outside of the buffer's size." });
        }

        voice.position = p_usize;
        voice.float_pos = position - p_usize as f64;

        Ok(())
    }

    pub fn read_buffer_stereo_f32(&mut self, buffer: &mut [f32]) {
        for sample_loc in (0..buffer.len()).step_by(2) {
            // 0 the buffer to make sure no garbage data is present.
            buffer[sample_loc] = 0.0;
            buffer[sample_loc + 1] = 0.0;

            for voice in &mut self.voices {
                if !voice.is_playing {
                    continue;
                }

                let internal_buffer = self.buffers[voice.buffer].as_ref().unwrap();

                let format = internal_buffer.format;
                let alignment = voice.alignment;

                voice.float_pos += voice.speed;
                let int_pos = voice.float_pos as usize;
                voice.position += int_pos;
                voice.float_pos -= int_pos as f64;

                let mut position = voice.position * alignment;

                if position >= voice.sample_end {
                    if voice.properties.looping {
                        voice.position = voice.properties.loop_start;
                        // TODO: Don't reset float_pos to 0, reset it to a proper value for accuracy in short loops.
                        voice.float_pos = 0.0;
                        position = voice.position * alignment;
                    } else {
                        voice.is_playing = false;
                        voice.buffer = usize::MAX;
                        voice.position = 0;
                        voice.float_pos = 0.0;

                        continue;
                    }
                }

                if position != voice.previous_position {
                    voice.lerp_pos = voice.previous_position;
                    voice.previous_position = position;
                }

                let volume = (self.volume * voice.properties.volume/* * (voice.position as f64 / self.fade_in as f64).clamp(0.0, 1.0)*/) as f32;

                let mut sample_l = Self::get_sample(position, &internal_buffer.data, internal_buffer.format.data_type);
                let mut sample_r = Self::get_sample(position + (internal_buffer.alignment / 2) * (format.channels - 1) as usize, &internal_buffer.data, format.data_type);

                let prev_l = Self::get_sample(voice.lerp_pos, &internal_buffer.data, internal_buffer.format.data_type);
                let prev_r = Self::get_sample(voice.lerp_pos + (internal_buffer.alignment / 2) * (format.channels - 1) as usize, &internal_buffer.data, internal_buffer.format.data_type);

                sample_l = Self::lerp(prev_l, sample_l, voice.float_pos);
                sample_r = Self::lerp(prev_r, sample_r, voice.float_pos);

                buffer[sample_loc] += f32::clamp(sample_l * volume, -1.0, 1.0);
                buffer[sample_loc + 1] += f32::clamp(sample_r * volume, -1.0, 1.0);
            }
        }
    }

    #[inline(always)]
    fn get_sample(position: usize, data: &[u8], dt: DataType) -> f32 {
        match dt {
            DataType::I8 => data[position] as f32 / i8::MAX as f32,
            DataType::U8 => (data[position] as f32 / u8::MAX as f32) * 2.0 - 1.0,
            DataType::I16 => (data[position] as i16 | (data[position + 1] as i16) << 8) as f32 / i16::MAX as f32,
            DataType::U16 => ((data[position] as u16 | (data[position + 1] as u16) << 8) as f32 / u16::MAX as f32) * 2.0 - 1.0,
            DataType::I32 => (data[position] as i32 | (data[position + 1] as i32) << 8 | (data[position + 2] as i32) << 16 | (data[position + 3] as i32) << 24) as f32 / i32::MAX as f32,
            DataType::F32 => unsafe { std::mem::transmute(data[position] as i32 | (data[position + 1] as i32) << 8 | (data[position + 2] as i32) << 16 | (data[position + 3] as i32) << 24) },
            DataType::F64 => todo!(),
        }
    }

    #[inline(always)]
    fn lerp(min: f32, max: f32, amount: f64) -> f32 {
        (((max as f64 - min as f64) * amount) + min as f64) as f32
    }
}

struct Buffer {
    data: Vec<u8>,
    format: AudioFormat,

    alignment: usize
}

struct Voice {
    // The buffer that is playing. If `is_playing` is false, this value SHOULD be usize::MAX.
    buffer: usize,

    // If true, the voice is playing. If false, then the voice is ignored.
    is_playing: bool,

    // The current position (in samples) of the voice.
    position: usize,
    
    // The current floating-point offset from the current position.
    float_pos: f64,

    // The amount at which to increase float_pos by
    speed: f64,

    // The current properties of this voice.
    properties: PlayProperties,

    // The end point in BYTES at which point mixr will either loop, or stop the voice.
    sample_end: usize,

    previous_position: usize,

    lerp_pos: usize,

    alignment: usize
}