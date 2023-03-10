use std::io;

use mixr::AudioFormat;

pub mod wav;
pub mod module;

pub trait SupportedFormat {
    fn load_memory(data: &[u8]) -> Result<Self, io::Error> where Self: Sized;

    fn format(&self) -> AudioFormat;

    // TODO: Improve this function, it's quite resource intensive.
    fn pcm_data(&self) -> Vec<u8>;
}