use std::time::Duration;

use mixr::{self, system::AudioSystem};
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
    let mut format = mixr::AudioFormat {
        sample_rate: None,
        channels: None
    };

    let mut system = mixr::system::AudioSystem::new(&mut format);

    let pcm = mixr::loaders::PCM::load_wav("/home/ollie/Music/Laxity - A question of luck.wav").unwrap();

    let length = pcm.data.len();
    let rate = pcm.format.sample_rate.unwrap();

    let buffer = system.create_buffer();
    system.update_buffer(&buffer, &pcm.data, &pcm.format);
    system.play_buffer(&buffer, 1.0, 1.15);

    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(2),
        samples: None
    };

    let device = audio.open_playback(None, &desired_spec, |_| {
        Audio {
            system: &mut system
        }
    }).unwrap();

    device.resume();

    std::thread::sleep(Duration::from_secs((((length as i32) / 4 / rate) - 1) as u64));
}