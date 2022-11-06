use std::collections::HashMap;
use crate::AudioFormat;

struct Buffer {
    has_data: bool,

    data: Option<Vec<u8>>,
    format: Option<AudioFormat>,

    playing: bool,
    chunk: u64,
    position: f64,
    speed: f64
}

pub struct AudioBuffer {
    handle: i32
}

pub struct AudioSystem {
    pub format: AudioFormat,
    
    buffers: HashMap<i32, Buffer>,
    current_handle: i32
}

impl AudioSystem {
    pub fn new(format: &mut AudioFormat) -> AudioSystem {
        format.channels.get_or_insert(2);
        format.sample_rate.get_or_insert(48000);

        AudioSystem { format: format.clone(), buffers: HashMap::new(), current_handle: 0 }
    }

    pub fn create_buffer(&mut self) -> AudioBuffer {
        let buffer = Buffer { has_data: false, data: None, format: None, playing: false, chunk: 0, position: 0.0, speed: 0.0 };
        self.buffers.insert(self.current_handle, buffer);
        
        let p_buffer = AudioBuffer { handle: self.current_handle };
        self.current_handle += 1;

        p_buffer
    }

    pub fn update_buffer(&mut self, buffer: &AudioBuffer, data: &Vec<u8>, format: &AudioFormat) {
        let mut i_buffer = self.buffers.get_mut(&buffer.handle).unwrap();

        i_buffer.data = Some(data.clone());
        i_buffer.format = Some(format.clone());
        i_buffer.has_data = true;
    }

    pub fn play_buffer(&mut self, buffer: &AudioBuffer, speed: f64) {
        let mut i_buffer = self.buffers.get_mut(&buffer.handle).unwrap();
        i_buffer.chunk = 0;
        i_buffer.position = 0.0;
        i_buffer.speed = i_buffer.format.as_ref().unwrap().sample_rate.unwrap() as f64 / self.format.sample_rate.unwrap() as f64;
        i_buffer.speed *= speed;
        i_buffer.playing = true;
        println!("{}", self.format.sample_rate.unwrap());
    }

    pub fn advance(&mut self) -> i16 {
        let mut result = 0;

        for (_, buffer) in self.buffers.iter_mut() {
            if !buffer.playing {
                continue;
            }
            let data = &buffer.data.as_mut().unwrap();
            buffer.position += buffer.speed;
            const CHUNK_SIZE: f64 = 48000.0;
            if buffer.position >= CHUNK_SIZE {
                buffer.chunk += 1;
                buffer.position = buffer.position - CHUNK_SIZE;
            }

            let mut pos = (buffer.position + buffer.chunk as f64 * CHUNK_SIZE) as usize;
            pos *= 2;
            pos -= pos % 4;
            result += data[pos] as i16 | ((data[pos + 1] as i16) << 8) as i16;
        }

        result
    }
}