use std::time::Duration;

use mixr::{self, system::AudioSystem, AudioFormat, ChannelProperties};
use sdl2::audio::{AudioSpecDesired, AudioCallback};

struct Audio<'a> {
    system: &'a mut AudioSystem
}

impl<'a> AudioCallback for Audio<'a> {
    type Channel = i16;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = self.system.advance();
        }
    }
}

#[test]
fn test_wav() {
    let format: Option<AudioFormat> = None;

    let mut system = mixr::system::AudioSystem::new(format, 2);

    let pcm = mixr::loaders::PCM::load_wav("/home/ollie/Music/Samples/Waveworld/soundstate/Samplepack18w/wah24.wav").unwrap();
    let buffer = system.create_buffer();
    system.update_buffer(buffer, &pcm.data, pcm.format).unwrap();
    system.play_buffer(buffer, 0, ChannelProperties { 
        volume: 1.0, 
        speed: 2.0, 
        panning: 0.5, 
        looping: true, 
        interpolation_type: mixr::InterpolationType::Linear
    }).unwrap();

    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(2 as u8),
        samples: Some(8192)
    };

    let device = audio.open_playback(None, &desired_spec, |_| {
        Audio {
            system: &mut system
        }
    }).unwrap();

    device.resume();

    //std::thread::sleep(Duration::from_secs((((length as i32) / 4 / rate) - 1) as u64));
    loop {
        std::thread::sleep(Duration::from_secs(5));
    }
}