use std::collections::HashMap;
use std::collections::VecDeque;
use crate::{AudioFormat, ChannelProperties};

#[derive(Debug)]
pub enum AudioErrorType {
    InvalidBuffer,
    InvalidChannel,
    NoChannels
}

#[derive(Debug)]
pub struct AudioError<'a> {
    pub error_type: AudioErrorType,
    pub description: &'a str
}

impl<'a> AudioError<'a> {
    pub fn new(error_type: AudioErrorType) -> Self {
        let description = match error_type {
            AudioErrorType::InvalidBuffer => "An invalid buffer was provided.",
            AudioErrorType::InvalidChannel => "An invalid channel was provided.",
            AudioErrorType::NoChannels => "No available channels.",
        };

        Self {
            error_type,
            description
        }
    }
}

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

    properties: ChannelProperties,
    queued: VecDeque<i32>
}

pub struct AudioSystem {
    pub format: AudioFormat,
    
    buffers: HashMap<i32, Buffer>,
    channels: Vec<Channel>,
    current_handle: i32,
    current_sample: u8,

    callback: Option<fn(u16, i32)>
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
                properties: ChannelProperties::default(),
                queued: VecDeque::new()
            });
        }

        AudioSystem { format: i_fmt, buffers: HashMap::new(), channels: v_channels, current_handle: 0, current_sample: 0, callback: None }
    }

    pub fn create_buffer(&mut self) -> i32 {
        let buffer = Buffer { has_data: false, data: Vec::new(), format: AudioFormat { channels: 0, sample_rate: 0, bits_per_sample: 0 } };
        self.buffers.insert(self.current_handle, buffer);
        
        let p_buffer = self.current_handle;
        self.current_handle += 1;

        p_buffer
    }

    pub fn delete_buffer(&mut self, buffer: i32) -> Result<(), AudioError> {
        for channel in self.channels.iter_mut() {
            if channel.buffer == buffer {
                channel.playing = false;
            }
        }

        self.buffers.remove(&buffer).ok_or(AudioError::new(AudioErrorType::InvalidBuffer))?;

        Ok(())
    }

    pub fn update_buffer(&mut self, buffer: i32, data: &[u8], format: AudioFormat) -> Result<(), AudioError> {

        let mut i_buffer = self.buffers.get_mut(&buffer).ok_or(AudioError::new(AudioErrorType::InvalidBuffer))?;

        i_buffer.data = data.to_vec();
        i_buffer.format = format;
        i_buffer.has_data = true;

        Ok(())
    }

    pub fn play_buffer(&mut self, buffer: i32, channel: u16, properties: ChannelProperties) -> Result<(), AudioError> {
        let i_buffer = self.buffers.get(&buffer).ok_or(AudioError::new(AudioErrorType::InvalidBuffer))?;
        let i_channel = self.channels.get_mut(channel as usize).ok_or(AudioError::new(AudioErrorType::InvalidChannel))?;

        i_channel.queued.clear();

        i_channel.chunk = 0;
        i_channel.position = 0.0;
        i_channel.properties = properties;
        i_channel.sample_rate = i_buffer.format.sample_rate;
        i_channel.speed = i_buffer.format.sample_rate as f64 / self.format.sample_rate as f64;
        i_channel.speed *= i_channel.properties.speed;
        i_channel.buffer = buffer;
        i_channel.playing = true;

        Ok(())
    }

    pub fn set_channel_properties(&mut self, channel: u16, properties: ChannelProperties) -> Result<(), AudioError> {
        let mut channel = &mut self.channels.get_mut(channel as usize).ok_or(AudioError::new(AudioErrorType::InvalidChannel))?;
        channel.properties = properties;
        channel.speed = self.buffers.get(&channel.buffer).ok_or(AudioError::new(AudioErrorType::InvalidBuffer))?.format.sample_rate as f64 / self.format.sample_rate as f64;
        channel.speed *= channel.properties.speed;

        Ok(())
    }

    pub fn play(&mut self, channel: u16) -> Result<(), AudioError> {
        let mut channel = &mut self.channels.get_mut(channel as usize).ok_or(AudioError::new(AudioErrorType::InvalidChannel))?;
        channel.playing = true;

        Ok(())
    }

    pub fn pause(&mut self, channel: u16) -> Result<(), AudioError> {
        let mut channel = &mut self.channels.get_mut(channel as usize).ok_or(AudioError::new(AudioErrorType::InvalidChannel))?;
        channel.playing = false;

        Ok(())
    }

    pub fn stop(&mut self, channel: u16) -> Result<(), AudioError> {
        let mut channel = &mut self.channels.get_mut(channel as usize).ok_or(AudioError::new(AudioErrorType::InvalidChannel))?;
        channel.playing = false;
        channel.position = 0.0;
        channel.chunk = 0;
        channel.queued.clear();

        Ok(())
    }

    pub fn set_buffer_finished_callback(&mut self, callback: fn(u16, i32)) {
        self.callback = Some(callback);
    }

    pub fn queue_buffer(&mut self, buffer: i32, channel: u16) -> Result<(), AudioError> {
        let channel = &mut self.channels.get_mut(channel as usize).ok_or(AudioError::new(AudioErrorType::InvalidChannel))?;
        if !self.buffers.contains_key(&buffer) { 
            return Err(AudioError::new(AudioErrorType::InvalidBuffer));
        }
        channel.queued.push_back(buffer);

        Ok(())
    }

    pub fn advance(&mut self) -> i16 {
        let mut result: i32 = 0;

        let mut current_channel = 0;
        for channel in self.channels.iter_mut() {
            if !channel.playing {
                continue;
            }

            const CHUNK_SIZE: f64 = 48000.0;

            let mut buffer = &self.buffers[&channel.buffer];
            let format = &buffer.format;
            let properties = &channel.properties;
            let fmt_channels = format.channels;
            let fmt_bps = format.bits_per_sample;
            let mut data = &buffer.data;

            let alignment = (fmt_bps / 8) as usize;

            let pos_f64 = channel.position + channel.chunk as f64 * CHUNK_SIZE as f64;
            let pos = pos_f64 as usize;

            let mut get_pos = pos * alignment * fmt_channels as usize;
            get_pos += self.current_sample as usize * alignment * (fmt_channels - 1) as usize;
            get_pos -= get_pos % alignment;
            
            let next_pos = if channel.speed < 1.0 { pos + channel.properties.interpolation_type as usize } else { pos };

            let mut get_next_pos = next_pos * alignment * fmt_channels as usize;
            get_next_pos += self.current_sample as usize * alignment * (fmt_channels - 1) as usize;
            get_next_pos -= get_next_pos % alignment;

            if get_next_pos >= data.len() {
                if properties.looping {
                    channel.chunk = 0;
                    let amount = channel.position - CHUNK_SIZE;
                    channel.position = if amount > 0.0 { amount } else { 0.0 };
                    if get_pos == get_next_pos {
                        get_pos = 0;
                    }

                    get_next_pos = 0;

                } else if channel.queued.len() > 0 {
                    if let Some(cb) = self.callback {
                        cb(current_channel, channel.buffer)
                    };

                    channel.buffer = channel.queued.pop_front().unwrap();
                    buffer = &self.buffers[&channel.buffer];
                    data = &buffer.data;

                    channel.chunk = 0;
                    let amount = channel.position - CHUNK_SIZE;
                    channel.position = if amount > 0.0 { amount } else { 0.0 };

                    if get_pos == get_next_pos {
                        get_pos = 0;
                    }

                    get_next_pos = 0;
                } else {
                    if let Some(cb) = self.callback {
                        cb(current_channel, channel.buffer)
                    };
                    channel.playing = false;
                    channel.chunk = 0;
                    channel.position = 0.0;
                    continue;
                }
            }

            if data.len() == 0 {
                continue;
            }

            let pan = f64::clamp(if self.current_sample == 0 { (1.0 - channel.properties.panning) * 2.0 } else { 1.0 - ((0.5 - channel.properties.panning)) * 2.0 }, 0.0, 1.0);
            
            let mut value = Self::get_sample(data, get_pos, fmt_bps);
            let value_next = Self::get_sample(data, get_next_pos, fmt_bps);

            value = Self::lerp(value, value_next, pos_f64 - pos as f64);
            value *= channel.properties.volume * pan;

            result += value as i32;

            if self.current_sample == 0 {
                channel.position += channel.speed;   
            }

            if channel.position >= CHUNK_SIZE {
                channel.chunk += 1;
                channel.position = channel.position - CHUNK_SIZE;
            }

            current_channel += 1;
        }

        self.current_sample += 1;
        self.current_sample = self.current_sample % 2;

        let result = i32::clamp(result, i16::MIN as i32, i16::MAX as i32) as i16;

        result
    }

    pub fn num_channels(&self) -> u16 {
        self.channels.len() as u16
    }

    pub fn is_playing(&self, channel: u16) -> bool {
        if let Some(channel) = self.channels.get(channel as usize) {
            channel.playing
        } else {
            false
        }
    }

    pub fn get_available_channel(&self) -> Result<u16, AudioError> {
        for (i, channel) in self.channels.iter().enumerate() {
            if !channel.playing {
                return Ok(i as u16);
            }
        }

        Err(AudioError::new(AudioErrorType::NoChannels))
    }

    #[inline(always)]
    fn get_sample(data: &[u8], pos: usize, fmt_bps: u8) -> f64 {
        match fmt_bps {
            16 => (data[pos] as i16 | ((data[pos + 1] as i16) << 8) as i16) as f64,
            8 => ((((data[pos] as i32) << 8) as i32) - i16::MAX as i32) as f64,
            _ => panic!("Invalid bits per sample.")
        }
    }

    #[inline(always)]
    fn lerp(value: f64, next: f64, amount: f64) -> f64 {
        amount * (next - value) + value
    }
}