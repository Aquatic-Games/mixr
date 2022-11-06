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
    pub fn load_wav(path: &str) -> Result<PCM, LoadError> {
        let mut reader = match BinaryReader::new(path) {
            Ok(reader) => reader,
            Err(error) => return Err(LoadError::new(error.to_string())),
        };

        if reader.read_string(4) != "RIFF" {
            return Err(LoadError::new(String::from("Given file is missing the \"RIFF\" file header. This means it is either not a wave file, or is the wrong wave type.")));
        }

        reader.read_i32();

        if reader.read_string(4) != "WAVE" {
            return Err(LoadError::new(String::from("The \"WAVE\" identifier was not found at its expected location. Currently, files with \"JUNK\" headers are not supported.")));
        }

        if reader.read_string(4) != "fmt " {
            return Err(LoadError::new(String::from("\"fmt \" identifier was not found at its expected location.")));
        }

        reader.read_i32();

        if reader.read_i16() != 1 {
            return Err(LoadError::new(String::from("Currently, only PCM formats are supported. This file may be compressed?")));
        }

        let channels = reader.read_i16();
        let sample_rate = reader.read_i32();

        reader.read_i32();
        reader.read_i16();

        let bits_per_sample = reader.read_i16();

        let format = AudioFormat {
            channels: Some(channels as u8),
            sample_rate: Some(sample_rate),
            bits_per_sample: Some(bits_per_sample as u8)
        };

        if reader.read_string(4) != "data" {
            return Err(LoadError::new(String::from("An error has occurred while reading the format data.")));
        }

        let data_size = reader.read_i32();
        let data = reader.read_bytes(data_size as usize);

        Ok(PCM { data, format })
    }
}