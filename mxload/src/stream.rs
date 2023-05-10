use std::{io::{Read, Seek}, fs::File};

use crate::AudioStream;
use mixr::{AudioFormat, DataType};
use skyetils::binary::BinaryReader;

pub struct Wav {
    reader: BinaryReader<File>,

    format: AudioFormat,

    data_size: usize,
    current_pos: usize
}

impl AudioStream for Wav {
    fn from_data(data: &[u8]) -> Self {
        todo!()
    }

    fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;

        let mut reader = BinaryReader::new(file);

        const RIFF: u32 = 0x46464952;
        const WAVE: u32 = 0x45564157;
        const FMT : u32 = 0x20746D66;
        const DATA: u32 = 0x61746164;

        if reader.read_u32().unwrap() != RIFF {
            panic!("Given file is not a valid wave file! (Missing RIFF header).");
        }

        reader.read_u32().unwrap(); // file size

        if reader.read_u32().unwrap() != WAVE {
            panic!("Given file is not a valid wave file! (Missing WAVE header).");
        }

        let mut format = AudioFormat::default();
        let mut data_size = 0;

        loop {
            match reader.read_u32().unwrap() {
                FMT => {
                    if reader.read_u32().unwrap() != 16 {
                        panic!("Malformed fmt header.");
                    }

                    let fmt_type = reader.read_u16().unwrap();

                    let channels = reader.read_u16().unwrap();

                    let sample_rate = reader.read_u32().unwrap();

                    reader.read_u32().unwrap(); // bytes per second
                    reader.read_u16().unwrap(); // bytes per sample
                    
                    let bits_per_sample = reader.read_u16().unwrap();

                    let data_type = match (fmt_type, bits_per_sample) {
                        (1, 8) => DataType::U8,
                        (1, 16) => DataType::I16,
                        (1, 32) => DataType::I32,
                        (3, 32) => DataType::F32,
                        _ => panic!("Unsupported audio format.")
                    };

                    format = AudioFormat { data_type, sample_rate, channels: channels as u8 };
                }

                DATA => {
                    data_size = reader.read_u32().unwrap() as usize;
                    break;
                }

                _ => {
                    let chunk_size = reader.read_u32().unwrap();
                    let curr_pos = reader.position().unwrap();
                    reader.set_position(curr_pos + chunk_size as usize).unwrap();
                }
            }
        }

        Ok(Self {
            reader,
            format,

            data_size,

            current_pos: 0
        })
    }

    fn format(&self) -> mixr::AudioFormat {
        self.format
    }

    fn get_buffer(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let read = self.reader.read_to_buf(buf)?;

        let read = if self.current_pos + read >= self.data_size {
            self.data_size - self.current_pos
        } else {
            read
        };

        self.current_pos += read;

        Ok(read)
    }

    fn get_pcm(&mut self) -> Result<Vec<u8>, std::io::Error> {
        self.reader.read_bytes(self.data_size)
    }
}