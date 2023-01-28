use std::fmt;

use crate::{AudioFormat, binary_reader::BinaryReader};

#[derive(Debug, Clone)]
pub struct LoadError {
    pub text: String
}

impl LoadError {
    pub fn new(text: String) -> LoadError {
        LoadError { text }
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to load PCM file: {}", self.text)
    }
}

pub struct PCM {
    pub data: Vec<u8>,
    pub format: AudioFormat
}

impl PCM {
    pub fn load_wav_path(path: &str) -> Result<PCM, LoadError> {
        let read = std::fs::read(path);

        if let Ok(data) = read {
            return Self::load_wav(&data);
        }

        return Err(LoadError::new(read.err().unwrap().to_string()));
        
    }

    pub fn load_wav(data: &[u8]) -> Result<PCM, LoadError> {
        let mut reader = BinaryReader::new(data);

        if reader.read_string(4) != "RIFF" {
            return Err(LoadError::new(String::from("Given file is missing the \"RIFF\" file header. This means it is either not a wave file, or is the wrong wave type.")));
        }

        reader.read_i32();

        if reader.read_string(4) != "WAVE" {
            return Err(LoadError::new(String::from("The \"WAVE\" identifier was not found at its expected location. Currently, files with \"JUNK\" headers are not supported.")));
        }

        let mut has_data = false;
        let mut format = AudioFormat::default();
        let mut data = Vec::new();

        while !has_data {
            let text = reader.read_string(4);

            let chunk = text.as_str();
            
            match chunk {
                "fmt " => {
                    reader.read_i32();

                    let a_fmt = reader.read_i16();

                    let floating_point = if a_fmt == 1 {
                        false
                    } else if a_fmt == 3 {
                        true
                    } else {
                        return Err(LoadError::new(String::from("Unrecognized audio format.")));
                    };

                    let channels = reader.read_i16();
                    let sample_rate = reader.read_i32();

                    reader.read_i32();
                    reader.read_i16();

                    let bits_per_sample = reader.read_i16();

                    format = AudioFormat {
                        channels: channels as u8,
                        sample_rate,
                        bits_per_sample: bits_per_sample as u8,
                        floating_point
                    };
                },

                "data" => {
                    let data_size = reader.read_i32();
                    data = reader.read_bytes(data_size as usize).to_vec();

                    has_data = true;
                }

                _ => reader.position += reader.read_i32() as usize
            }
        }

        

        Ok(PCM { data, format })
    }
}

pub trait Stream {
    fn format(&self) -> AudioFormat;

    fn buffer_size(&self) -> usize;

    fn buffer(&mut self) -> &[u8];
}

pub fn load_stream_path(path: &str) -> Result<Box<dyn Stream + Send>, LoadError> {
    let read = std::fs::read(path);

    if let Ok(data) = read {
        return Ok(load_stream(&data));
    }

    return Err(LoadError::new(read.err().unwrap().to_string()));
}

pub fn load_stream(data: &[u8]) -> Box<dyn Stream + Send> {
    let mut reader = BinaryReader::new(data);
    //if reader.read_string("RIFF") {
        const DEFAULT_BUFFER_SIZE: usize = 22050 * 2 * 2;
        let mut buffer = Vec::with_capacity(DEFAULT_BUFFER_SIZE);
        for _ in 0..DEFAULT_BUFFER_SIZE {
            buffer.push(0);
        }

        return Box::new(PcmStream { pcm: PCM::load_wav(data).unwrap(), buffer, buffer_pos: 0 })
    //}
}

pub struct PcmStream {
    pcm: PCM,
    buffer: Vec<u8>,
    buffer_pos: usize
}

impl Stream for PcmStream {
    fn buffer(&mut self) -> &[u8] {
        let mut decoded = 0;

        for value in self.buffer.iter_mut() {
            *value = self.pcm.data[self.buffer_pos];
            self.buffer_pos += 1;
            if self.buffer_pos >= self.pcm.data.len() {
                break;
            }
            decoded += 1;
        }

        &self.buffer[0..decoded]
    }

    fn buffer_size(&self) -> usize {
        self.buffer.len()
    }

    fn format(&self) -> AudioFormat {
        self.pcm.format
    }
}