use mixr::AudioFormat;
use polymod::{track::Track, track_player::TrackPlayer};

use crate::SupportedFormat;

pub struct Module {
    format: AudioFormat,
    track:  Track
}

impl SupportedFormat for Module {
    fn load_memory(data: &[u8]) -> Result<Self, std::io::Error> where Self: Sized {
        Ok(Self {
            format: AudioFormat { channels: 2, sample_rate: 48000, format_type: mixr::FormatType::F32 },
            track: Track::from_it(data)?
        })
    }

    fn format(&self) -> mixr::AudioFormat {
        self.format
    }

    fn pcm_data(&self) -> Vec<u8> {
        let mut player = TrackPlayer::new(&self.track);
        let length_in_samples = (self.track.length_in_seconds * self.format.sample_rate as f64 * 2.0) as usize;

        let mut data = Vec::with_capacity(length_in_samples * self.format.bytes_per_sample() as usize);

        for _ in 0..length_in_samples {
            let sample = (player.advance() as f32).to_le_bytes();

            data.push(sample[0]);
            data.push(sample[1]);
            data.push(sample[2]);
            data.push(sample[3]);
        }

        data
    }
}