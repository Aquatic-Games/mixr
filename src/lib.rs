pub mod system;
pub mod loaders;
pub mod binary_reader;
mod cmixr;

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct AudioFormat {
    pub channels: u8,
    pub sample_rate: i32,
    pub bits_per_sample: u8
}