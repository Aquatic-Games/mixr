use mixr::AudioFormat;

pub mod stream;

pub trait AudioStream {
    fn from_data(data: &[u8]) -> Self;

    fn from_file(path: &str) -> Result<Self, std::io::Error> where Self : Sized;

    fn format(&self) -> AudioFormat;

    fn metadata(&self) -> &TrackMetadata;

    fn get_buffer(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error>;

    fn get_pcm(&mut self) -> Result<Vec<u8>, std::io::Error>;
}

#[derive(Debug, Clone, Default)]
pub struct TrackMetadata {
    pub title:  Option<String>,
    pub artist: Option<String>,
    pub album:  Option<String>,
    pub year:   Option<String>,
    pub genre:  Option<String>
}