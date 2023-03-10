// mixr, piegfx 2023.
// Play a wave file.
// Simply input your path, and the program will run until the sound has finished.

use clap::Parser;
use mixr::{self, ChannelProperties, BufferDescription, DataType, system::AudioSystem};
use mxload::SupportedFormat;
use sdl2::audio::{AudioSpecDesired, AudioCallback};

#[derive(Parser)]
struct CommandArgs {
    path: String,

    #[arg(short, long, default_value_t = 1.0)]
    volume: f64,

    #[arg(short, long, default_value_t = 1.0)]
    speed: f64,

    #[arg(short, long, default_value_t = 0.5)]
    panning: f64,

    #[arg(long, default_value_t = false)]
    looping: bool
}

fn main() {
    let args = CommandArgs::parse();
    let path = args.path.as_str();
    let volume = args.volume;
    let speed = args.speed;
    let panning = args.panning;
    let looping = args.looping;

    const SAMPLE_RATE: i32 = 48000;
    let mut system = AudioSystem::new(SAMPLE_RATE, 1);

    let file = if let Ok(data) = std::fs::read(path) {
        data
    } else {
        println!("Failed to find file \"{path}\".");
        return;
    };

    let wav = mxload::wav::Wav::load_memory(&file).unwrap();
    //let wav = mxload::module::Module::load_memory(&file).unwrap();
    let buffer = system.create_buffer(BufferDescription { data_type: DataType::Pcm, format: wav.format() }, Some(&wav.pcm_data()));

    system.play_buffer(buffer, 0, ChannelProperties {
        volume,
        speed,
        panning,
        looping,
        interpolation: mixr::InterpolationType::Linear,
        loop_start: 0,
        loop_end: -1,
    }).unwrap();

    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(SAMPLE_RATE),
        channels: Some(2),
        samples: Some(8192)
    };

    let device = audio.open_playback(None, &desired_spec, |_| {
        Audio {
            system: &mut system
        }
    }).unwrap();

    device.resume();

    ctrlc::set_handler(move || std::process::exit(0)).unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if !system.is_playing(0) {
            std::process::exit(0);
        }
    }
}

struct Audio<'a> {
    system: &'a mut AudioSystem
}

impl<'a> AudioCallback for Audio<'a> {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        self.system.advance_buffer(out);
    }
}