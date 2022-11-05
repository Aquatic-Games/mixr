#[derive(Clone, Debug)]
pub struct AudioFormat {
    pub channels: Option<i32>,
    pub sample_rate: Option<i32>,
}

pub struct AudioSystem {
    pub format: AudioFormat
}

impl AudioSystem {
    pub fn new(format: &mut AudioFormat) -> AudioSystem {
        format.channels.get_or_insert(2);
        format.sample_rate.get_or_insert(48000);

        AudioSystem { format: format.clone() }
    }

    pub fn advance() -> i16 {
        0
    }
}