use std::collections::HashMap;
use crate::AudioFormat;

struct Buffer {
    has_data: bool,

    data: Option<Vec<u8>>,
    format: Option<AudioFormat>,
}

struct Channel {
    buffer: i32,
    playing: bool,
    chunk: u64,
    position: f64,
    speed: f64,
    volume: f32
}

pub struct AudioBuffer {
    handle: i32
}

pub struct AudioSystem {
    pub format: AudioFormat,
    
    buffers: HashMap<i32, Buffer>,
    channels: Vec<Channel>,
    current_handle: i32
}

impl AudioSystem {
    pub fn new(format: &mut AudioFormat, channels: u16) -> AudioSystem {
        format.channels.get_or_insert(2);
        format.sample_rate.get_or_insert(48000);

        let mut v_channels = Vec::with_capacity(channels as usize);
        for _ in 0..channels {
            v_channels.push(Channel {
                buffer: -1,
                playing: false,
                chunk: 0,
                position: 0.0,
                speed: 0.0,
                volume: 0.0,
            });
        }

        AudioSystem { format: format.clone(), buffers: HashMap::new(), channels: v_channels, current_handle: 0 }
    }

    pub fn create_buffer(&mut self) -> AudioBuffer {
        let buffer = Buffer { has_data: false, data: None, format: None };
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

    pub fn play_buffer(&mut self, channel: u16, buffer: &AudioBuffer, volume: f32, speed: f64) {
        let i_buffer = self.buffers.get(&buffer.handle).unwrap();
        let i_channel = self.channels.get_mut(channel as usize).unwrap();
        i_channel.chunk = 0;
        i_channel.position = 0.0;
        i_channel.speed = i_buffer.format.as_ref().unwrap().sample_rate.unwrap() as f64 / self.format.sample_rate.unwrap() as f64;
        i_channel.speed *= speed;
        i_channel.volume = volume;
        i_channel.playing = true;
        i_channel.buffer = buffer.handle;
        println!("{}", self.format.sample_rate.unwrap());
    }

    pub fn advance(&mut self) -> i16 {
        let mut result: i32 = 0;

        for channel in self.channels.iter_mut() {
            if !channel.playing {
                continue;
            }
            let buffer = self.buffers.get(&channel.buffer).unwrap();
            let data = buffer.data.as_ref().unwrap();
            channel.position += channel.speed;
            const CHUNK_SIZE: f64 = 48000.0;
            if channel.position >= CHUNK_SIZE {
                channel.chunk += 1;
                channel.position = channel.position - CHUNK_SIZE;
            }

            let mut pos = (channel.position + channel.chunk as f64 * CHUNK_SIZE) as usize;
            pos *= 2;
            pos -= pos % 4;
            result += ((data[pos] as i16 | ((data[pos + 1] as i16) << 8) as i16) as f32 * channel.volume) as i32;
        }

        let result = i32::clamp(result, i16::MIN as i32, i16::MAX as i32) as i16;

        result
    }
}