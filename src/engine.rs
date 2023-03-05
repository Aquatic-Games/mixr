use std::io;

use crate::system::AudioSystem;

pub struct AudioEngine {
    system: AudioSystem
}

impl AudioEngine {
    pub fn new(sample_rate: i32, max_sounds: u16) -> Self {
        let sdl = sdl2::init().unwrap();

        Self {
            system: AudioSystem::new(sample_rate, max_sounds)
        }
    }

    pub fn system(&self) -> &AudioSystem {
        &self.system
    }
}

pub struct Sound<'a> {
    engine: &'a AudioEngine
}

impl<'a> Sound<'a> {
    //pub fn from_file(engine: &AudioEngine) -> Result<Self, io::Error> {
    //    Ok(()))
    //}


}