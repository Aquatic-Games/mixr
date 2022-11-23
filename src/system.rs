use std::collections::HashMap;
use crate::{AudioFormat, ChannelProperties};

struct Buffer {
    has_data: bool,

    data: Vec<u8>,
    format: AudioFormat,
}

struct Channel {
    buffer: i32,
    playing: bool,
    chunk: u64,
    position: f64,
    sample_rate: i32,
    speed: f64,

    properties: ChannelProperties
}

pub struct AudioSystem {
    pub format: AudioFormat,
    
    buffers: HashMap<i32, Buffer>,
    channels: Vec<Channel>,
    current_handle: i32,
    current_sample: u8
}

impl AudioSystem {
    pub fn new(format: Option<AudioFormat>, channels: u16) -> AudioSystem {
        let i_fmt = if format.is_none() {
            AudioFormat {
                channels: 2,
                sample_rate: 48000,
                bits_per_sample: 16
            }
        } else { format.unwrap() };

        let mut v_channels = Vec::with_capacity(channels as usize);
        for _ in 0..channels {
            v_channels.push(Channel {
                buffer: -1,
                playing: false,
                chunk: 0,
                position: 0.0,
                sample_rate: 0,
                speed: 0.0,
                properties: ChannelProperties::default()
            });
        }

        AudioSystem { format: i_fmt, buffers: HashMap::new(), channels: v_channels, current_handle: 0, current_sample: 0 }
    }

    pub fn create_buffer(&mut self) -> i32 {
        let buffer = Buffer { has_data: false, data: Vec::new(), format: AudioFormat { channels: 0, sample_rate: 0, bits_per_sample: 0 } };
        self.buffers.insert(self.current_handle, buffer);
        
        let p_buffer = self.current_handle;
        self.current_handle += 1;

        p_buffer
    }

    pub fn update_buffer(&mut self, buffer: i32, data: &Vec<u8>, format: AudioFormat) {

        let mut i_buffer = self.buffers.get_mut(&buffer).unwrap();

        i_buffer.data = data.clone();
        i_buffer.format = format;
        i_buffer.has_data = true;
    }

    pub fn play_buffer(&mut self, buffer: i32, channel: u16, properties: ChannelProperties) {
        let i_buffer = self.buffers.get(&buffer).unwrap();
        let i_channel = self.channels.get_mut(channel as usize).unwrap();

        i_channel.chunk = 0;
        i_channel.position = 0.0;
        i_channel.properties = properties;
        i_channel.sample_rate = i_buffer.format.sample_rate;
        i_channel.speed = i_buffer.format.sample_rate as f64 / self.format.sample_rate as f64;
        i_channel.speed *= i_channel.properties.speed;
        i_channel.playing = true;
        i_channel.buffer = buffer;
    }

    pub fn set_channel_properties(&mut self, channel: u16, properties: ChannelProperties) {
        let mut channel = &mut self.channels[channel as usize];
        channel.properties = properties;
        channel.speed = self.buffers[&channel.buffer].format.sample_rate as f64 / self.format.sample_rate as f64;
        channel.speed *= channel.properties.speed;
    }

    pub fn play(&mut self, channel: u16) {
        self.channels[channel as usize].playing = true;
    }

    pub fn pause(&mut self, channel: u16) {
        self.channels[channel as usize].playing = false;
    }

    pub fn stop(&mut self, channel: u16) {
        let mut channel = &mut self.channels[channel as usize];
        channel.playing = false;
        channel.position = 0.0;
        channel.chunk = 0;
    }

    pub fn advance(&mut self) -> i16 {
        let mut result: i32 = 0;

        for channel in self.channels.iter_mut() {
            if !channel.playing {
                continue;
            }

            const CHUNK_SIZE: f64 = 48000.0;

            let buffer = &self.buffers[&channel.buffer];
            let format = &buffer.format;
            let properties = &channel.properties;
            let fmt_channels = format.channels;
            let fmt_bps = format.bits_per_sample;
            let data = &buffer.data;

            let alignment = fmt_bps / 8;

            let mut pos = (channel.position + channel.chunk as f64 * CHUNK_SIZE) as usize;
            pos *= alignment as usize;
            pos += if fmt_channels == 2 { self.current_sample as usize } else { 0 };
            pos *= fmt_channels as usize;
            pos -= pos % alignment as usize;
            //pos -= pos % 2 as usize;

            if pos >= data.len() {
                if properties.looping {
                    channel.chunk = 0;
                    let amount = channel.position - CHUNK_SIZE;
                    channel.position = if amount > 0.0 { amount } else { 0.0 };

                    // todo: perhaps move this into its own function to stop duplicated code
                    // make sure to measure perf to make sure it's not changed though
                    pos = (channel.position + channel.chunk as f64 * CHUNK_SIZE) as usize;
                    pos *= alignment as usize;
                    pos += if fmt_channels == 2 { self.current_sample as usize } else { 0 };
                    pos *= fmt_channels as usize;
                    pos -= pos % alignment as usize;
                } else {
                    channel.playing = false;
                }
            }

            let pan = f64::clamp(if self.current_sample == 0 { (1.0 - channel.properties.panning) * 2.0 } else { 1.0 - ((0.5 - channel.properties.panning)) * 2.0 }, 0.0, 1.0);

            unsafe {

                if fmt_bps == 16 {
                    result += ((*data.get_unchecked(pos) as i16 | ((*data.get_unchecked(pos + 1) as i16) << 8) as i16) as f64 * channel.properties.volume * pan) as i32;
                }
                else if fmt_bps == 8 {
                    result += (((((*data.get_unchecked(pos) as i32) << 8) as i32) - i16::MAX as i32) as f64 * channel.properties.volume * pan) as i32;
                }

            }

            if self.current_sample == 0 {
                channel.position += channel.speed;   
            }

            if channel.position >= CHUNK_SIZE {
                channel.chunk += 1;
                channel.position = channel.position - CHUNK_SIZE;
            }
        }

        self.current_sample += 1;
        self.current_sample = self.current_sample % 2;

        let result = i32::clamp(result, i16::MIN as i32, i16::MAX as i32) as i16;

        result
    }
}