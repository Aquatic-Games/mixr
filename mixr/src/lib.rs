#[derive(Debug, Clone, Copy)]
pub enum ErrorType {
    InvalidBuffer,
    InvalidVoice,
    InvalidValue,

    Other
}

#[derive(Debug, Clone, Copy)]
pub struct MixrError<'a> {
    pub e_type:  ErrorType,
    pub message: &'a str
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct AudioFormat {
    pub data_type: DataType,
    pub sample_rate: u32,
    pub channels: u8
}

#[derive(Debug, Clone, Copy)]
pub struct BufferDescription {
    pub format: AudioFormat
}

#[derive(Debug, Clone, Copy)]
pub struct PlayProperties {
    pub volume: f64,
    pub speed: f64,
    
    pub looping: bool,
    pub loop_start: usize,
    pub loop_end: usize
}

impl Default for PlayProperties {
    fn default() -> Self {
        Self { volume: 1.0, speed: 1.0, looping: false, loop_start: 0, loop_end: 0 }
    }
}

#[derive(Clone, Copy)]
pub struct AudioBuffer {
    id: usize
}

pub struct AudioSystem {
    pub volume: f64,

    sample_rate: u32,

    buffers: Vec<Option<Buffer>>,
    voices: Vec<Voice>
}

impl AudioSystem {
    pub fn new(sample_rate: u32, voices: u16) -> Self {
        let mut channels = Vec::with_capacity(voices as usize);

        for _ in 0..voices {
            channels.push(Voice { 
                buffer: None,

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

            sample_rate,

            buffers: Vec::new(),
            voices: channels
        }
    }

    pub fn create_buffer<T>(&mut self, description: BufferDescription, data: Option<&[T]>) -> AudioBuffer {
        let data: Vec<u8> = if let Some(dat) = data {
            let length = dat.len() * std::mem::size_of::<T>();
            unsafe { Vec::from_raw_parts(dat.as_ptr() as *mut _, length, length) }
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

        AudioBuffer {
            id: self.buffers.len() - 1
        }
    }

    pub fn play_buffer(&mut self, buffer: AudioBuffer, voice: u16, properties: PlayProperties) -> Result<(), MixrError> {
        let mut voice = self.voices.get_mut(voice as usize).ok_or(MixrError { e_type: ErrorType::InvalidVoice, message: "Voice was out of range." })?;
        let internal_buffer = self.buffers[buffer.id].as_ref().ok_or(MixrError { e_type: ErrorType::InvalidBuffer, message: "Buffer was not valid. Has it been deleted?" })?;

        if properties.looping && properties.loop_end != 0 {
            if properties.loop_end - properties.loop_start == 0 {
                return Err(MixrError { e_type: ErrorType::InvalidValue, message: "Loop size cannot total 0. (loop start and loop end values are the same)" });
            } else if properties.loop_end < properties.loop_start {
                return Err(MixrError { e_type: ErrorType::InvalidValue, message: "Loop end position cannot be less than the loop start position." });
            }
        }

        voice.buffer = Some(buffer.id);
        voice.position = 0;
        voice.float_pos = 0.0;

        let align = f64::ceil(properties.speed);

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

    pub fn read_buffer_stereo_f32(&mut self, buffer: &mut [f32]) {
        for sample_loc in (0..buffer.len()).step_by(2) {
            // 0 the buffer to make sure no garbage data is present.
            buffer[sample_loc] = 0.0;
            buffer[sample_loc + 1] = 0.0;

            for voice in &mut self.voices {
                let internal_buffer = if let Some(buf) = voice.buffer {
                    self.buffers[buf].as_ref().unwrap()
                } else {
                    continue;
                };

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
                        voice.buffer = None;
                        voice.position = 0;
                        voice.float_pos = 0.0;
                        voice.speed = 0.0;

                        continue;
                    }
                }

                if position != voice.previous_position {
                    voice.lerp_pos = voice.previous_position;
                    voice.previous_position = position;
                }

                let volume = (self.volume * voice.properties.volume) as f32;

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
            DataType::I32 => todo!(),
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
    // The buffer that is playing. If None, then the voice is ignored.
    buffer: Option<usize>,

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