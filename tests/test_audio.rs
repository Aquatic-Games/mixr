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
    let format = AudioFormat { channels: 2, sample_rate: 48000, bits_per_sample: 16 };

    let mut system = mixr::system::AudioSystem::new(Some(format.clone()), 2);
    system.master_volume = 1.0;

    let pcm1 = mixr::loaders::PCM::load_wav("/home/ollie/Music/Always There.wav").unwrap();
    //let pcm1 = mixr::loaders::PCM::load_wav("/home/ollie/Music/others/kf-main-start.wav").unwrap();
    let pcm2 = mixr::loaders::PCM::load_wav("/home/ollie/Music/others/kf-main-loop.wav").unwrap();

    let buffer1 = system.create_buffer();
    let buffer2 = system.create_buffer();

    system.update_buffer(buffer1, &pcm1.data, pcm1.format).unwrap();
    system.update_buffer(buffer2, &pcm2.data, pcm2.format).unwrap();

    system.play_buffer(buffer1, 0, ChannelProperties { 
        volume: 1.0, 
        speed: 1.15, 
        panning: 0.5, 
        looping: false, 
        interpolation_type: mixr::InterpolationType::Linear
    }).unwrap();

    system.queue_buffer(buffer2, 0).unwrap();

    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(format.sample_rate),
        channels: Some(format.channels),
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