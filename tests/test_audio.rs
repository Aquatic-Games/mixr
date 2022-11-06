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
        sample_rate: Some(48000),
        channels: None,
        bits_per_sample: None
    };

    let mut system = mixr::system::AudioSystem::new(&mut format, 32);

    let pcm1 = mixr::loaders::PCM::load_wav("/home/ollie/Music/Always There MONO.wav").unwrap();
    let pcm2 = mixr::loaders::PCM::load_wav("/home/ollie/Music/Samples/LowRes/Always There-8khz.wav").unwrap();

    let length = pcm1.data.len();
    let rate = pcm1.format.sample_rate.unwrap();

    let buffer1 = system.create_buffer();
    system.update_buffer(buffer1, &pcm1.data, &pcm1.format);

    let buffer2 = system.create_buffer();
    system.update_buffer(buffer2, &pcm2.data, &pcm2.format);


    system.play_buffer(0, buffer2, 1.0, 1.0);
    //system.play_buffer(3, &buffer2, 1.0, 1.45);

    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(format.sample_rate.unwrap()),
        channels: Some(format.channels.unwrap() as u8),
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