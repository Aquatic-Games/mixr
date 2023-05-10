use clap::Parser;
use mixr::{AudioSystem, BufferDescription, PlayProperties, PlayState};
use mxload::{stream::Wav, AudioStream};
use sdl2::audio::{AudioSpecDesired, AudioCallback};

#[derive(Parser)]
struct CliArgs {
    #[arg(required = true)]
    paths: Vec<String>,

    #[arg(short, long, default_value_t = 1.0)]
    volume: f64,

    #[arg(short, long, default_value_t = 1.0)]
    speed: f64,

    #[arg(long, default_value_t = false)]
    looping: bool
}

fn main() {
    println!("mxplay 0.1.0\npiegfx 2023");

    let args = CliArgs::parse();
    let speed = args.speed;
    let volume = args.volume;
    let looping = args.looping;

    let mut wav = if let Ok(wav) = Wav::from_file(&args.paths[0]) {
        wav
    } else {
        println!("Could not find file with path \"{}\"!", args.paths[0]);
        return;
    };

    let pcm = wav.get_pcm().unwrap();

    let mut system = AudioSystem::new(48000, 1);

    let buffer = system.create_buffer(BufferDescription {
        format: wav.format()
    }, Some(&pcm)).unwrap();

    system.play_buffer(buffer, 0, PlayProperties {
        speed,
        volume,
        looping,
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
        println!(); // print a newline
        std::process::exit(0);
    }).unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        if system.get_voice_state(0).unwrap() != PlayState::Playing {
            std::process::exit(0);
        }
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
