use mixr::{self, ChannelProperties, system::{AudioSystem}};
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

fn main() {
    let path = std::env::args().nth(1).expect("A path is required.");
    let path = path.as_str();

    let mut system = AudioSystem::new(None, 1);

    let pcm = mixr::loaders::PCM::load_wav(path).expect("A valid path is required. Make sure if it contains spaces, you surround it with quotes.");

    let buffer = system.create_buffer();
    system.update_buffer(buffer, &pcm.data, pcm.format).unwrap();

    system.play_buffer(buffer, 0, ChannelProperties {
        volume: 1.0,
        speed: 1.0,
        panning: 0.5,
        looping: false,
        interpolation_type: mixr::InterpolationType::Linear,
        loop_start: 0,
        loop_end: -1,
    }).unwrap();

    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(system.format.sample_rate),
        channels: Some(system.format.channels),
        samples: Some(8192)
    };

    let device = audio.open_playback(None, &desired_spec, |_| {
        Audio {
            system: &mut system
        }
    }).unwrap();

    device.resume();

    ctrlc::set_handler(move || std::process::exit(0)).unwrap();

    //std::thread::sleep(Duration::from_secs((((length as i32) / 4 / rate) - 1) as u64));
    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}