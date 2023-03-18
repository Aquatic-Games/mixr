// mixr, piegfx 2023.
// Play a wave file.
// Simply input your path, and the program will run until the sound has finished.

use clap::Parser;
use mixr::{self, ChannelProperties, BufferDescription, DataType, system::AudioSystem};
use mxload::SupportedFormat;
use sdl2::audio::{AudioSpecDesired, AudioCallback};

#[derive(Parser)]
struct CommandArgs {
    paths: Vec<String>,

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
    println!("mxplay 0.1.0\n(C) piegfx 2023");

    const SAMPLE_RATE: i32 = 48000;
    let mut system: AudioSystem = AudioSystem::new(SAMPLE_RATE, 1);

    let args = CommandArgs::parse();
    let path = args.paths[0].as_str();
    let volume = args.volume;
    let speed = args.speed;
    let panning = args.panning;
    let looping = args.looping;

    println!("Loading \"{path}\"...");

    let file = if let Ok(data) = std::fs::read(path) {
        data
    } else {
        println!("Failed to find file \"{path}\".");
        return;
    };

    let wav = mxload::wav::Wav::load_memory(&file).unwrap();
    //let wav = mxload::module::Module::load_memory(&file).unwrap();
    let buffer = system.create_buffer(BufferDescription { data_type: DataType::Pcm, format: wav.format() }, Some(&wav.pcm_data()));

    let properties = ChannelProperties {
        volume,
        speed,
        panning,
        looping: if args.paths.len() == 1 { looping } else { false },
        interpolation: mixr::InterpolationType::Linear,
        loop_start: 0,
        loop_end: -1,
    };

    system.play_buffer(buffer, 0, properties).unwrap();

    println!("Playing \"{path}\"");

    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(SAMPLE_RATE),
        channels: Some(2),
        samples: Some(8192)
    };

    let device = audio.open_playback(None, &desired_spec, |_| {
        Audio {
            properties,
            looping,
            current_path: 0,
            paths: args.paths,
            system: &mut system
        }
    }).unwrap();

    device.resume();

    ctrlc::set_handler(|| {
        println!();
        std::process::exit(0)
    }).unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        if !system.is_playing(0) {
            std::process::exit(0);
        }
    }
}

struct Audio<'a> {
    properties: ChannelProperties,
    looping: bool,
    system: &'a mut AudioSystem,
    current_path: usize,
    paths: Vec<String>
}

impl<'a> AudioCallback for Audio<'a> {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        self.system.advance_buffer(out);

        while let Some((channel, buffer)) = self.system.pop_finished_buffer() {
            self.system.delete_buffer(buffer).unwrap();
            self.current_path += 1;
            if self.current_path >= self.paths.len() {
                if self.looping {
                    self.current_path = 0;
                } else {
                    return;
                }
            }
            let path = &self.paths[self.current_path];

            println!("Loading \"{path}\"...");

            let file = if let Ok(data) = std::fs::read(path) {
                data
            } else {
                println!("Failed to find file \"{path}\".");
                return;
            };

            let wav = mxload::wav::Wav::load_memory(&file).unwrap();
            let buffer = self.system.create_buffer(BufferDescription { data_type: DataType::Pcm, format: wav.format() }, Some(&wav.pcm_data()));
            self.system.play_buffer(buffer, channel, self.properties).unwrap();

            println!("Playing \"{path}\"");
        }
    }
}