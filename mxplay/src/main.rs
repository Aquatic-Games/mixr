use clap::Parser;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use mixr::{AudioBuffer, AudioSystem, BufferDescription, PlayProperties};
use mixr::stream::{AudioStream, Stream};

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
    let paths = vec![
        "/home/skye/Music/house2.ogg".to_string(),
        "/home/skye/Music/SCD/1-20 Quartz Quadrant 'G' Mix JP.wav".to_string()
    ];

    let desired = AudioSpecDesired {
        freq: Some(SAMPLE_RATE as i32),
        channels: Some(2),
        samples: Some(8192),
    };

    let sdl = sdl2::init().unwrap();
    let audio = sdl.audio().unwrap();

    let device = audio.open_playback(None, &desired, |_| {
        Audio::new(paths)
    }).unwrap();

    device.resume();

    ctrlc::set_handler(|| {
        std::process::exit(0);
    }).unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

const SAMPLE_RATE: u32 = 48000;
const NUM_BUFFERS: usize = 2;
const BUFFER_SIZE: usize = 48000;
const VOICE: u16 = 0;

struct Audio {
    system: AudioSystem,
    paths: Vec<String>,

    buffer: AudioBuffer,

    current_sound: usize
}

impl Audio {
    pub fn new(paths: Vec<String>) -> Self {
        if paths.is_empty() {
            panic!("Empty path provided.");
        }

        let mut system = AudioSystem::new(SAMPLE_RATE, 1);

        let mut stream = Stream::from_file(paths[0].as_str()).unwrap();
        let format = stream.format();
        let pcm = stream.get_pcm().unwrap();

        let buffer = system.create_buffer(BufferDescription { format }, Some(&pcm)).unwrap();
        system.play_buffer(buffer, VOICE, PlayProperties::default()).unwrap();

        Self {
            system,
            paths,

            buffer,

            current_sound: 0
        }
    }
}

impl AudioCallback for Audio {
    type Channel = f32;

    fn callback(&mut self, buf: &mut [Self::Channel]) {
        self.system.read_buffer_stereo_f32(buf);
    }
}

unsafe impl Send for Audio { }