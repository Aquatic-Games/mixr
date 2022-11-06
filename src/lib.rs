pub mod system;
pub mod loaders;
pub mod binary_reader;

#[derive(Clone, Debug)]
pub struct AudioFormat {
    pub channels: Option<i32>,
    pub sample_rate: Option<i32>,
}