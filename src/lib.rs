pub mod system;
pub mod loaders;
pub mod binary_reader;

#[derive(Clone, Debug, Default)]
pub struct AudioFormat {
    pub channels: Option<u8>,
    pub sample_rate: Option<i32>,
    pub bits_per_sample: Option<u8>
}