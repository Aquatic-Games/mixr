use std::ffi::{c_char, c_float, c_int, c_uint, c_void, CString};
use std::fs::File;
use std::io::{Error, ErrorKind};

use crate::{AudioFormat, DataType};
use skyetils::binary::BinaryReader;

pub trait AudioStream: Send + Sync {
    fn from_data(data: &[u8]) -> Self where Self : Sized;

    fn from_file(path: &str) -> Result<Self, std::io::Error> where Self : Sized;

    fn format(&self) -> AudioFormat;

    fn metadata(&self) -> &TrackMetadata;

    fn get_buffer(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error>;

    fn get_pcm(&mut self) -> Result<Vec<u8>, std::io::Error>;

    fn pcm_length(&self) -> usize;
}

#[derive(Debug, Clone, Default)]
pub struct TrackMetadata {
    pub title:  Option<String>,
    pub artist: Option<String>,
    pub album:  Option<String>,
    pub year:   Option<String>,
    pub genre:  Option<String>
}

pub struct Stream {
    stream: Box<dyn AudioStream>
}

impl AudioStream for Stream {
    fn from_data(data: &[u8]) -> Self where Self: Sized {
        todo!()
    }

    fn from_file(path: &str) -> Result<Self, Error> where Self: Sized {
        let stream = Vorbis::from_file(path)?;

        Ok(Self {
            stream: Box::new(stream)
        })
    }

    fn format(&self) -> AudioFormat {
        self.stream.format()
    }

    fn metadata(&self) -> &TrackMetadata {
        self.stream.metadata()
    }

    fn get_buffer(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.stream.get_buffer(buf)
    }

    fn get_pcm(&mut self) -> Result<Vec<u8>, Error> {
        self.stream.get_pcm()
    }

    fn pcm_length(&self) -> usize {
        self.stream.pcm_length()
    }
}

pub struct Wav {
    reader: BinaryReader<File>,

    format: AudioFormat,
    metadata: TrackMetadata,

    data_size: usize,
    current_pos: usize
}

impl AudioStream for Wav {
    fn from_data(data: &[u8]) -> Self {
        todo!()
    }

    fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        if let Ok(metadata) = file.metadata() {
            if metadata.is_dir() {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Given file is a directory."));
            }
        }

        let mut reader = BinaryReader::new(file);

        const RIFF: u32 = 0x46464952;
        const WAVE: u32 = 0x45564157;
        const FMT : u32 = 0x20746D66;
        const DATA: u32 = 0x61746164;

        // Metadata identifiers
        const LIST: u32 = 0x5453494C;
        const INFO: u32 = 0x4F464E49;
        const INAM: u32 = 0x4D414E49;
        const IART: u32 = 0x54524149;
        const IPRD: u32 = 0x44524049;
        const ICRD: u32 = 0x44524349;
        const IGNR: u32 = 0x524E4749;

        if reader.read_u32().unwrap() != RIFF {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Given file is not a valid wave file! (Missing RIFF header)."));
        }

        reader.read_u32().unwrap(); // file size

        if reader.read_u32().unwrap() != WAVE {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Given file is not a valid wave file! (Missing WAVE header)."));
        }

        let mut format = AudioFormat::default();
        let mut metadata = TrackMetadata::default();
        let mut data_size = 0;

        loop {
            match reader.read_u32().unwrap() {
                FMT => {
                    if reader.read_u32().unwrap() != 16 {
                        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Malformed fmt header."));
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
                        _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unsupported audio format."))
                    };

                    format = AudioFormat { data_type, sample_rate, channels: channels as u8 };
                }

                DATA => {
                    data_size = reader.read_u32().unwrap() as usize;
                    break;
                },

                // Read metadata info.
                /*LIST => {
                    let size = reader.read_u32().unwrap();
                    let pos = reader.position().unwrap();

                    while reader.position().unwrap() < pos + size as usize {
                        match reader.read_u32().unwrap() {
                            0 =>  { reader.read_u8().unwrap(); },

                            INFO => {},

                            INAM => {
                                let size = reader.read_u32().unwrap();

                                let bytes = reader.read_bytes(size as usize).unwrap();

                                metadata.title = Some(String::from_utf8(bytes).unwrap());
                            },

                            _ => {


                                let chunk_size = reader.read_u32().unwrap();
                                println!("{chunk_size}");
                                let curr_pos = reader.position().unwrap();
                                reader.set_position(curr_pos + chunk_size as usize).unwrap();
                            }
                        }
                    }

                    println!("{}", pos + size as usize);
                },*/

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
            metadata,

            data_size,

            current_pos: 0
        })
    }

    fn format(&self) -> crate::AudioFormat {
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

    fn pcm_length(&self) -> usize {
        self.data_size
    }

    fn metadata(&self) -> &TrackMetadata {
        &self.metadata
    }
}

struct Vorbis {
    vorbis: *mut StbVorbis,
    format: AudioFormat
}

impl AudioStream for Vorbis {
    fn from_data(data: &[u8]) -> Self where Self: Sized {
        todo!()
    }

    fn from_file(path: &str) -> Result<Self, Error> where Self: Sized {
        let path = CString::new(path).unwrap();
        let c_path = path.as_c_str();

        unsafe {
            let mut error = 0;
            let vorbis = stb_vorbis_open_filename(c_path.as_ptr(), &mut error, std::ptr::null());

            if vorbis == std::ptr::null_mut() {
                return Err(Error::new(ErrorKind::InvalidData, format!("Failed to load vorbis. Error code {error}.")));
            }

            let info = stb_vorbis_get_info(vorbis);

            let format = AudioFormat {
                data_type: DataType::F32,
                sample_rate: info.sample_rate,
                channels: info.channels as u8,
            };

            Ok(Self {
                vorbis,
                format
            })
        }
    }

    fn format(&self) -> AudioFormat {
        self.format
    }

    fn metadata(&self) -> &TrackMetadata {
        todo!()
    }

    fn get_buffer(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        unsafe {
            let amount = stb_vorbis_get_samples_float_interleaved(self.vorbis, self.format.channels as c_int, buf.as_mut_ptr() as *mut _, (buf.len() / 4) as c_int) as usize * self.format.channels as usize;
            if amount < buf.len() / 4 {
                panic!("expected {}, got {amount}.", buf.len() / 4);
            }
            Ok(amount)
        }
    }

    fn get_pcm(&mut self) -> Result<Vec<u8>, Error> {
        let length = self.pcm_length();
        let mut vec = Vec::with_capacity(length);
        unsafe { vec.set_len(length) };
        self.get_buffer(&mut vec)?;

        Ok(vec)
    }

    fn pcm_length(&self) -> usize {
        // multiply by 4 as I believe pcm_length should be in bytes not samples.
        unsafe { stb_vorbis_stream_length_in_samples(self.vorbis) as usize * 4 * self.format.channels as usize }
    }
}

impl Drop for Vorbis {
    fn drop(&mut self) {
        unsafe {
            stb_vorbis_close(self.vorbis);
        }
    }
}

unsafe impl Send for Vorbis {}
unsafe impl Sync for Vorbis {}

#[repr(C)]
struct StbVorbisInfo {
    pub sample_rate: c_uint,
    pub channels: c_int,

    pub setup_memory_required: c_uint,
    pub setup_temp_memory_required: c_uint,
    pub temp_memory_required: c_uint,

    pub max_frame_size: c_int
}

type StbVorbis = c_void;

extern "C" {
    fn stb_vorbis_open_filename(filename: *const c_char, error: *mut c_int, stb_vorbis_alloc: *const c_void) -> *mut StbVorbis;

    fn stb_vorbis_close(f: *mut StbVorbis);

    fn stb_vorbis_get_info(f: *mut StbVorbis) -> StbVorbisInfo;

    fn stb_vorbis_stream_length_in_samples(f: *mut StbVorbis) -> c_uint;

    fn stb_vorbis_get_samples_float_interleaved(f: *mut StbVorbis, channels: c_int, buffer: *mut c_float, num_floats: c_int) -> c_int;
}