use crate::AudioSystem;

pub struct AudioEngine {
    system: AudioSystem
}

impl AudioEngine {
    pub fn new(sample_rate: u32, max_sounds: u16) -> Self {
        Self {
            system: AudioSystem::new(sample_rate, max_sounds)
        }
    }
}

pub struct Sound<'a> {
    engine: &'a AudioEngine
}