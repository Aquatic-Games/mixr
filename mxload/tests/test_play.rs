use mixr::*;
use sdl2::audio::{AudioSpecDesired, AudioCallback};
use mxload::{stream::Wav, AudioStream};

#[test]
fn test_playback() {
    let mut system = AudioSystem::new(48000, 16);

    let mut wav = Wav::from_file("/home/ollie/Music/necros_-_introspection.wav").unwrap();

    println!("{:#?}", wav.format());

    /*let mut full_buffer = Vec::new();

    let mut buf: Vec<u8> = std::iter::repeat(0).take(24000).collect();

    let mut total_amount = 0;

    loop {
        let amount = wav.get_buffer(&mut buf).unwrap();

        full_buffer.append(&mut buf);

        total_amount += amount;

        if amount < 24000 {
            break;
        }

        unsafe { buf.set_len(24000) };
    }

    full_buffer.truncate(total_amount);*/

    let full_buffer = wav.get_pcm().unwrap();

    let buffer = system.create_buffer(BufferDescription {
        format: wav.format()
    }, Some(&full_buffer)).unwrap();

    system.play_buffer(buffer, 0, PlayProperties {
        speed: 1.15,
        looping: true,
        ..Default::default()
    }).unwrap();

    let sdl = sdl2::init().unwrap();

    let audio = sdl.audio().unwrap();

    let spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(2),
        samples: Some(512),
    };

    let device = audio.open_playback(None, &spec, |_| {
        Audio {
            system: &mut system
        }
    }).unwrap();

    device.resume();

    ctrlc::set_handler(|| {
        std::process::exit(0);
    }).unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

struct Audio<'a> {
    system: &'a mut AudioSystem
}

impl AudioCallback for Audio<'_> {
    type Channel = f32;

    fn callback(&mut self, buf: &mut [Self::Channel]) {
        self.system.read_buffer_stereo_f32(buf);
    }
}