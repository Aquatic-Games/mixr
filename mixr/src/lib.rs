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
    pub speed: f64
}

impl Default for PlayProperties {
    fn default() -> Self {
        Self { volume: 1.0, speed: 1.0 }
    }
}

#[derive(Clone, Copy)]
pub struct AudioBuffer {
    id: usize
}

struct Buffer {
    data: Vec<u8>,
    format: AudioFormat,

    alignment: usize
}

pub struct AudioSystem {
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

                properties: PlayProperties::default()
            });
        }

        Self {
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

    pub fn play_buffer(&mut self, buffer: AudioBuffer, voice: u16, properties: PlayProperties) {
        let mut voice = self.voices.get_mut(voice as usize).expect("Voice was out of range of voices!");
        let internal_buffer = self.buffers[buffer.id].as_ref().expect("Buffer is not a valid buffer!");

        voice.buffer = Some(buffer.id);
        voice.position = 0;
        voice.float_pos = 0.0;

        voice.speed = (internal_buffer.format.sample_rate as f64 / self.sample_rate as f64) * properties.speed;
    }

    pub fn read_buffer_stereo_f32(&mut self, buffer: &mut [f32]) {
        for sample_loc in (0..buffer.len()).step_by(2) {
            for voice in &mut self.voices {
                let internal_buffer = if let Some(buf) = voice.buffer {
                    self.buffers[buf].as_ref().unwrap()
                } else {
                    continue;
                };

                voice.float_pos += voice.speed;
                let int_pos = voice.float_pos as usize;
                voice.position += int_pos;
                voice.float_pos -= int_pos as f64;

                let position = voice.position * internal_buffer.alignment;

                let sample_l = Self::get_sample(position, &internal_buffer.data, internal_buffer.format.data_type);
                let sample_r = Self::get_sample(position + internal_buffer.alignment / 2, &internal_buffer.data, internal_buffer.format.data_type);

                buffer[sample_loc] = sample_l;
                buffer[sample_loc + 1] = sample_r;
            }
        }
    }

    #[inline(always)]
    fn get_sample(position: usize, data: &[u8], dt: DataType) -> f32 {
        match dt {
            DataType::I8 => data[position] as f32 / i8::MAX as f32,
            DataType::U8 => (data[position] as f32 / u8::MAX as f32) * 2.0 - 1.0,
            DataType::I16 => ((data[position] as i16 | (data[position + 1] as i16) << 8) as f32 / i16::MAX as f32),
            DataType::U16 => ((data[position] as u16 | (data[position + 1] as u16) << 8) as f32 / u16::MAX as f32) * 2.0 - 1.0,
            DataType::I32 => todo!(),
            DataType::F32 => unsafe { std::mem::transmute(data[position] as i32 | (data[position + 1] as i32) << 8 | (data[position + 2] as i32) << 16 | (data[position + 3] as i32) << 24) },
            DataType::F64 => todo!(),
        }
    }
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
    properties: PlayProperties
}