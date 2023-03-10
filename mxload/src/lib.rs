use std::io;

use mixr::AudioFormat;

pub mod wav;

pub trait SupportedFormat {
    fn load_memory(data: &[u8]) -> Result<Self, io::Error> where Self: Sized;

    fn format(&self) -> AudioFormat;

    fn pcm_data(&self) -> &[u8];
}