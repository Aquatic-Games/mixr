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
    data: Vec<u8>,
    format: AudioFormat,
    bytes_per_sample: u8,
    length_in_samples: usize
}

struct Channel {
    sample_rate: i32,
    speed: f64,

    buffer: i32,
    prev_buffer: i32,

    chunk: u64,
    position: f64,

    prev_curr_sample: usize,
    prev_sample: usize,

    playing: bool,
    properties: ChannelProperties,
    queued: VecDeque<i32>
}

pub struct AudioSystem {
    pub format: AudioFormat,
    pub master_volume: f64,

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
                sample_rate: 0,
                speed: 0.0,

                buffer: -1,
                prev_buffer: -1,

                chunk: 0,
                position: 0.0,

                prev_curr_sample: 0,
                prev_sample: 0,

                playing: false,
                properties: ChannelProperties::default(),
                queued: VecDeque::new()
            });
        }

        AudioSystem { 
            format: i_fmt, 
            buffers: HashMap::new(), 
            channels: v_channels, 
            current_handle: 0, 
            current_sample: 0, 
            callback: None,

            master_volume: 1.0
        }
    }

    pub fn create_buffer(&mut self) -> i32 {
        let buffer = Buffer { 
            data: Vec::new(), 
            format: AudioFormat { channels: 0, sample_rate: 0, bits_per_sample: 0 }, 
            length_in_samples: 0,
            bytes_per_sample: 0 
        };

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

    pub fn update_buffer<T: Sized>(&mut self, buffer: i32, data: &[T], format: AudioFormat) -> Result<(), AudioError> {

        let mut i_buffer = self.buffers.get_mut(&buffer).ok_or(AudioError::new(AudioErrorType::InvalidBuffer))?;

        i_buffer.data = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * std::mem::size_of::<T>()).to_vec() };
        
        // As the data is stored as bytes, we need to convert this to a byte value.
        // A bits per sample of 8 = 1 byte. 16 bits = 2 bytes, etc.
        let bytes_per_sample = format.bits_per_sample / 8;

        i_buffer.length_in_samples = data.len() / bytes_per_sample as usize / format.channels as usize;
        i_buffer.bytes_per_sample = bytes_per_sample;

        i_buffer.format = format;

        Ok(())
    }

    pub fn play_buffer(&mut self, buffer: i32, channel: u16, properties: ChannelProperties) -> Result<(), AudioError> {
        let i_buffer = self.buffers.get(&buffer).ok_or(AudioError::new(AudioErrorType::InvalidBuffer))?;
        let i_channel = self.channels.get_mut(channel as usize).ok_or(AudioError::new(AudioErrorType::InvalidChannel))?;

        i_channel.queued.clear();

        i_channel.chunk = 0;
        i_channel.position = 0.0;
        i_channel.prev_sample = 0;
        i_channel.prev_curr_sample = 0;

        i_channel.properties = properties;
        if i_channel.properties.loop_end == -1 {
            i_channel.properties.loop_end = self.buffers[&buffer].length_in_samples as i32;
        }

        i_channel.sample_rate = i_buffer.format.sample_rate;
        i_channel.speed = i_buffer.format.sample_rate as f64 / self.format.sample_rate as f64;
        i_channel.speed *= i_channel.properties.speed;
        i_channel.buffer = buffer;
        i_channel.prev_buffer = buffer;
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
        let mut result: f64 = 0.0;

        let mut current_channel = 0;
        for channel in self.channels.iter_mut() {
            // Do not attempt to mix channels that are not playing.
            if !channel.playing {
                continue;
            }

            // Chunk size denotes how many samples a chunk is. In this case, 48000 samples.
            const CHUNK_SIZE: f64 = 48000.0;

            let mut curr_buffer = &self.buffers[&channel.buffer];
            let prev_buffer = &self.buffers[&channel.prev_buffer];

            let properties = &channel.properties;
            let curr_format = &curr_buffer.format;
            let prev_format = &prev_buffer.format;

            // Calculate the current and previous sample.
            // Sample != position in array
            let mut curr_sample_f64 = channel.chunk as f64 * CHUNK_SIZE + channel.position;
            let mut curr_sample = curr_sample_f64 as usize;

            if curr_sample >= properties.loop_end as usize {
                if properties.looping {
                    // Looping just returns the channel back to 0.
                    channel.chunk = (properties.loop_start as f64 / CHUNK_SIZE) as u64;
                    channel.position = if channel.chunk == 0 { properties.loop_start as f64 } else { properties.loop_start as f64 / channel.chunk as f64 - CHUNK_SIZE };
                    curr_sample_f64 = channel.chunk as f64 * CHUNK_SIZE + channel.position;
                    curr_sample = curr_sample_f64 as usize;

                } else if channel.queued.len() > 0 {
                    if let Some(cb) = self.callback {
                        cb(current_channel, channel.buffer);
                    }

                    channel.chunk = 0;
                    channel.position = if CHUNK_SIZE > channel.position { 0.0 } else { channel.position - CHUNK_SIZE };
                    curr_sample_f64 = channel.chunk as f64 * CHUNK_SIZE + channel.position;
                    curr_sample = curr_sample_f64 as usize;

                    channel.buffer = channel.queued.pop_front().unwrap();
                    curr_buffer = &self.buffers[&channel.buffer];

                } else {
                    if let Some(cb) = self.callback {
                        cb(current_channel, channel.buffer);
                    }

                    channel.playing = false;
                    continue;
                }
            }

            if channel.prev_curr_sample != curr_sample {
                channel.prev_sample = channel.prev_curr_sample;
                channel.prev_curr_sample = curr_sample;
            }

            let prev_sample = channel.prev_sample;

            let curr_data = &curr_buffer.data;
            let prev_data = &prev_buffer.data;
            
            let curr_bps = curr_buffer.bytes_per_sample as usize;
            let prev_bps = prev_buffer.bytes_per_sample as usize;

            let mut curr_pos = curr_sample * curr_bps * curr_format.channels as usize;
            curr_pos += self.current_sample as usize * (curr_format.channels - 1) as usize * curr_bps;
            curr_pos -= curr_pos % curr_bps * curr_format.channels as usize;

            let mut curr_value = Self::get_sample(curr_data, curr_pos, curr_format.bits_per_sample);

            match properties.interpolation_type {
                crate::InterpolationType::None => {},
                crate::InterpolationType::Linear => {
                    let mut prev_pos = prev_sample * prev_bps * prev_format.channels as usize;
                    prev_pos += self.current_sample as usize * (prev_format.channels - 1) as usize * prev_bps;
                    prev_pos -= prev_pos % prev_bps * prev_format.channels as usize;

                    let prev_value = Self::get_sample(prev_data, prev_pos, prev_format.bits_per_sample);

                    curr_value = Self::lerp(prev_value, curr_value, curr_sample_f64 - curr_sample as f64);
                }
            }

            let pan = f64::clamp(if self.current_sample == 0 { (1.0 - properties.panning) * 2.0 } else { 1.0 - ((0.5 - properties.panning)) * 2.0 }, 0.0, 1.0);

            result += curr_value * pan * properties.volume;

            channel.prev_buffer = channel.buffer;

            // Advance by the channel's speed, but only when both stereo channels have been mixed.
            if self.current_sample == 0 {
                channel.position += channel.speed;
                if channel.position >= CHUNK_SIZE {
                    channel.chunk += 1;
                    channel.position -= CHUNK_SIZE;
                }
            }

            current_channel += 1;
        }

        self.current_sample += 1;
        self.current_sample = self.current_sample % 2;

        let result = f64::clamp(result * self.master_volume, i16::MIN as f64, i16::MAX as f64) as i16;

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