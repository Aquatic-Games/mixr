use mixr::*;
use sdl2::audio::{AudioSpecDesired, AudioCallback};

#[test]
fn test_mixr() {
    let mut system = AudioSystem::new(48000, 16);

    let data = std::fs::read("/home/ollie/Music/TESTFILES/amba2-32bitfloat.raw").unwrap();
    //let data2 = std::fs::read("/home/ollie/Music/TESTFILES/nixonspace-16bitshort.raw").unwrap();

    let buffer = system.create_buffer(BufferDescription {
        format: AudioFormat {
            data_type: DataType::F32,
            sample_rate: 48000,
            channels: 2
        }
    }, Some(&data));

    /*let buffer2 = system.create_buffer(BufferDescription {
        format: AudioFormat {
            data_type: DataType::I16,
            sample_rate: 48000,
            channels: 2
        }
    }, Some(&data2));*/

    system.play_buffer(buffer, 0, PlayProperties {
        speed: 2.0,
        looping: true,
        ..Default::default()
    }).unwrap();

    /*system.play_buffer(buffer2, 1, PlayProperties {
        speed: 1.0,
        looping: true,
        ..Default::default()
    }).unwrap();*/

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