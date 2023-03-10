use std::io;
use mixr::{AudioFormat, binary_reader::BinaryReader, FormatType};

use crate::SupportedFormat;

pub struct Wav {
    format: AudioFormat,
    data:   Vec<u8>
}

impl SupportedFormat for Wav {
    fn load_memory(data: &[u8]) -> Result<Self, io::Error> {
        let mut reader = BinaryReader::new(data);

        if reader.read_string(4) != "RIFF" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Given file is missing the \"RIFF\" file header. This means it is either not a wave file, or is the wrong wave type."));
        }

        reader.read_i32();

        if reader.read_string(4) != "WAVE" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "The \"WAVE\" identifier was not found at its expected location. Currently, files with \"JUNK\" headers are not supported."));
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
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unrecognized audio format."));
                    };

                    let channels = reader.read_i16();
                    let sample_rate = reader.read_i32();

                    reader.read_i32();
                    reader.read_i16();

                    let bits_per_sample = reader.read_i16();

                    let format_type = match bits_per_sample {
                        8 => FormatType::U8,
                        16 => FormatType::I16,
                        32 => if floating_point { FormatType::F32 } else { FormatType::I32 },
                        _ => return Err(io::Error::new(io::ErrorKind::Unsupported, format!("{bits_per_sample}-bit audio is not yet supported.").as_str()))
                    };

                    format = AudioFormat {
                        channels: channels as u8,
                        sample_rate,
                        format_type
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

        

        Ok(Wav { format, data })
    }

    fn format(&self) -> AudioFormat {
        self.format
    }

    fn pcm_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}