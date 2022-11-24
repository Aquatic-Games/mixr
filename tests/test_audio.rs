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
    let mut format: Option<AudioFormat> = None;

    let mut system = mixr::system::AudioSystem::new(format, 2);

    let pcm1 = mixr::loaders::PCM::load_wav("/home/ollie/Music/dr6.wav").unwrap();
    let pcm2 = mixr::loaders::PCM::load_wav("/home/ollie/Music/r-59.wav").unwrap();

    let length = pcm1.data.len();
    let rate = pcm1.format.sample_rate;

    let buffer1 = system.create_buffer();
    system.update_buffer(buffer1, &pcm1.data, pcm1.format);

    let buffer2 = system.create_buffer();
    system.update_buffer(buffer2, &pcm2.data, pcm2.format);

    system.play_buffer(buffer2, 0, ChannelProperties { volume: 1.0, speed: 1.0, panning: 0.5, looping: true  });

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

    std::thread::sleep(Duration::from_secs((((length as i32) / 4 / rate) - 1) as u64));
}