use std::{io::{Read, Seek}, fs::File};

use crate::AudioStream;
use mixr::{AudioFormat, DataType};
use skyetils::binary::BinaryReader;

pub struct Wav {
    reader: BinaryReader<File>,

    format: AudioFormat
}

impl AudioStream for Wav {
    fn from_data(data: &[u8]) -> Self {
        todo!()
    }

    fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap();

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
                    println!("{}", reader.position().unwrap());
                    break;
                }

                _ => {
                    let chunk_size = reader.read_u32().unwrap();
                    let curr_pos = reader.position().unwrap();
                    reader.set_position(curr_pos + chunk_size as usize).unwrap();
                }
            }
        }

        Self {
            reader,
            format
        }
    }

    fn format(&self) -> mixr::AudioFormat {
        self.format
    }

    fn get_buffer(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.reader.read_to_buf(buf)
    }
}