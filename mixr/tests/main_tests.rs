use mixr::*;
use sdl2::audio::{AudioSpecDesired, AudioCallback};

#[test]
fn test_mixr() {
    let mut system = AudioSystem::new(48000, 16);

    let data1 = std::fs::read("/home/skye/Music/TESTFILES/dedune-start-32bitfloat.raw").unwrap();
    let data2 = std::fs::read("/home/skye/Music/TESTFILES/nixonspace-16bitshort.raw").unwrap();

    let buffer1 = system.create_buffer(BufferDescription {
        format: AudioFormat {
            data_type: DataType::F32,
            sample_rate: 44100,
            channels: 2
        }
    }, Some(&data1)).unwrap();

    let buffer2 = system.create_buffer(BufferDescription {
        format: AudioFormat {
            data_type: DataType::I16,
            sample_rate: 48000,
            channels: 2
        }
    }, Some(&data2)).unwrap();

    system.play_buffer(buffer1, 0, PlayProperties {
        speed: 1.0,
        looping: true,
        ..Default::default()
    }).unwrap();

    system.queue_buffer(buffer2, 0).unwrap();

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

    let mut num_loops = 0;
    loop {
        let pos_secs = system.get_position(0).unwrap();
        let state = system.get_voice_state(0).unwrap();

        /*if num_loops == 2 {
            system.set_voice_state(0, PlayState::Paused).unwrap();
        }

        if num_loops == 4 {
            system.set_voice_state(0, PlayState::Playing).unwrap();
        }

        if num_loops == 10 {
            system.set_voice_state(0, PlayState::Stopped).unwrap();
        }

        if num_loops == 14 {
            system.set_voice_state(0, PlayState::Playing).unwrap();
        }*/

        println!("{:?}: {}, {}s", state, system.get_position_samples(0).unwrap(), pos_secs);
        std::thread::sleep(std::time::Duration::from_secs(1));

        if state == PlayState::Stopped {
            break;
        }

        num_loops += 1;
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