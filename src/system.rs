use std::collections::HashMap;
use crate::AudioFormat;

struct Buffer {
    has_data: bool,

    data: Option<Vec<u8>>,
    format: Option<AudioFormat>,

    playing: bool,
    position: usize
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
        let buffer = Buffer { has_data: false, data: None, format: None, playing: false, position: 0 };
        self.buffers.insert(self.current_handle, buffer);
        
        let p_buffer = AudioBuffer { handle: self.current_handle };
        self.current_handle += 1;

        p_buffer
    }

    pub fn update_buffer(&mut self, buffer: &AudioBuffer, data: Vec<u8>, format: AudioFormat) {
        let mut i_buffer = self.buffers.get_mut(&buffer.handle).unwrap();

        i_buffer.data = Some(data);
        i_buffer.format = Some(format);
        i_buffer.has_data = true;
    }

    pub fn play_buffer(&mut self, buffer: &AudioBuffer) {
        let mut i_buffer = self.buffers.get_mut(&buffer.handle).unwrap();
        i_buffer.position = 0;
        i_buffer.playing = true;
    }

    pub fn advance(&mut self) -> i16 {
        let mut result = 0;

        for (_, buffer) in self.buffers.iter_mut() {
            let data = &buffer.data.as_mut().unwrap();
            let pos = buffer.position;
            result += data[pos] as i16 | ((data[pos + 1] as i16) << 8) as i16;

            buffer.position += 2;
        }

        result
    }
}